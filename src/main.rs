use std::fs;
use what_time::{get_cmd_line_args, get_local_time, parse_config};

fn main() {
    // get command line args:
    let args = get_cmd_line_args();

    // read and parse config file:
    let config = fs::read_to_string(args.config_file).expect("Could not load config file");
    let zones = parse_config(&config);

    // convert current time to friend's timezone, and print:
    let local_time = get_local_time(&args.name, zones);
    println!("{}", local_time);
}
