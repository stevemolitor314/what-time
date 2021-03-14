use clap::App;
use std::fs;
use what_time::{default_config, get_local_time, parse_config};

fn main() {
    // specify command line args:
    let matches = App::new("what-time")
        .version("1.0")
        .about("Prints current time for friends in other time zones, in their time zone.")
        .args_from_usage(
            "-c --config=[FILE] 'Friends timezones file, defaults to ~/.what-time'
             <NAME> 'The name of the friend to report the current time for.'",
        )
        .get_matches();

    // parse command line args:
    let default_config = default_config();
    let config_file = matches.value_of("config").unwrap_or(&default_config);
    let name = matches.value_of("NAME").unwrap();

    // read and parse config file:
    let config = fs::read_to_string(config_file).expect("Could not load config file");
    let zones = parse_config(&config);

    // convert current time to friend's timezone, and print:
    let local_time = get_local_time(name, zones);
    println!("{}", local_time);
}
