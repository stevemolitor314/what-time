use chrono::prelude::Utc;
use reqwest::blocking::Client;
use std::{collections::HashMap, fs, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WhatTimeError {
    #[error("Error reading config file from disk")]
    ReadConfigFromDiskError(#[from] io::Error),

    #[error("Error reading config via HTTP request")]
    ReadConfigViaHttpError(#[from] reqwest::Error),

    #[error("Error parsing timezone string")]
    ParseTzStringError(String),
}

/// Parsed command line arguments
pub struct CmdLineArgs {
    /// Friend name to report time for
    pub name: String,
    /// Path to configuration file
    pub config_path_or_url: String,
}

pub fn get_cmd_line_args() -> CmdLineArgs {
    // specify command line args:
    let matches = clap::App::new("what-time")
        .version("0.2.0")
        .about("Prints current time for friends in other time zones, in their time zone.")
        .args_from_usage(
            "-c --config=[FILE] 'Friends timezones file, defaults to ~/.what-time. Can be a URL starting with /https?/, in which case the config will be fetched from that URL.'
             <NAME> 'The name of the friend to report the current time for.'",
        )
        .get_matches();

    // parse command line args:
    let default_config = default_config();
    let config_file = matches.value_of("config").unwrap_or(&default_config);
    let name = matches.value_of("NAME").unwrap();

    CmdLineArgs {
        name: name.to_string(),
        config_path_or_url: config_file.to_string(),
    }
}

/// Get the default config file path, from `$HOME/.what-time`
pub fn default_config() -> String {
    let home: String = match std::env::var("HOME") {
        Ok(val) => val,
        _ => ".".to_string(), // err, just use current directory if no $HOME env
    };
    format!("{}/.what-time", home)
}

fn is_url(path: &str) -> bool {
    path.starts_with("http") || path.starts_with("https")
}

/// Get config file as string.
pub fn get_config(path_or_url: &str) -> Result<String, WhatTimeError> {
    if is_url(path_or_url) {
        get_config_from_http_request(path_or_url)
    } else {
        get_config_from_file_path(path_or_url)
    }
}

fn get_config_from_file_path(config_path: &str) -> Result<String, WhatTimeError> {
    let text = fs::read_to_string(config_path)?;
    Ok(text)
}

fn get_config_from_http_request(url: &str) -> Result<String, WhatTimeError> {
    let client = Client::new();
    let text = client.get(url).send()?.text()?;
    Ok(text)
}

/// Parse config file string into HashMap of name / time zone pairs.
/// Each line in the config file is a white space separated pair of name
/// and timezones.
pub fn parse_config(config: &str) -> HashMap<String, String> {
    let mut zones: HashMap<String, String> = HashMap::new();
    let lines = config.lines();
    for line in lines {
        let mut iter = line.split_whitespace();
        // Just skip over any blank config lines:
        if let Some(name) = iter.next() {
            match iter.next() {
                Some(zone) => {
                    let name_lower = name.to_lowercase().to_string();
                    zones.insert(name_lower, zone.to_string());
                }
                None => {
                    eprintln!(
                        "Ignoring config line with name {} but no timezone value",
                        name
                    );
                }
            }
        }
    }
    zones
}

/// Convert current time to friend's time zone, format, and return as string
pub fn get_local_time(name: &str, zones: HashMap<String, String>) -> Result<String, WhatTimeError> {
    let tz_string = &zones[&name.to_lowercase()];
    let parse_result = tz_string.parse();

    let tz: chrono_tz::Tz;
    match parse_result {
        Ok(t) => tz = t,
        Err(err) => return Err(WhatTimeError::ParseTzStringError(err)),
    }

    let now = Utc::now();
    let local_time = now.with_timezone(&tz).format("%a %I:%M %p").to_string();
    Ok(local_time)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let config = "
tom America/Los_Angeles
sally NZ
";

        let zones = parse_config(config);
        assert_eq!(zones["sally"], "NZ");
        assert_eq!(zones["tom"], "America/Los_Angeles");
    }

    #[test]
    fn test_parse_config_makes_names_lower_case() {
        let config = "Bob   America/Chicago";
        let zones = parse_config(config);
        assert_eq!(zones["bob"], "America/Chicago");
    }

    #[test]
    fn test_is_url() {
        assert!(is_url("http://"));
        assert!(is_url("https://"));
        assert!(!is_url("./what-time"))
    }
}
