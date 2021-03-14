use chrono::prelude::Utc;
use chrono_tz::*;
use std::{collections::HashMap, env};

/// Get the default config file path, from `$HOME/.what-time`.
pub fn default_config() -> String {
    let home: String = match env::var("HOME") {
        Ok(val) => val,
        _ => String::from("."), // err, just use current directory if no $HOME env
    };
    format!("{}/.what-time", home)
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

/// Convert current time to friend's time zone, format and return as string
pub fn get_local_time(name: &str, zones: HashMap<String, String>) -> String {
    let tz_string = &zones[&name.to_lowercase()];
    let err_msg = format!("Invalid time zone '{}'", tz_string);
    let tz: Tz = tz_string.parse().expect(&err_msg);
    let now = Utc::now();
    now.with_timezone(&tz).format("%a %I:%M %p").to_string()
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
}
