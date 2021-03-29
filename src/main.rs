use std::process;
use what_time::*;

fn main() {
    // get command line args:
    let CmdLineArgs {
        config_path_or_url,
        name,
    } = get_cmd_line_args();

    // read and parse config file:
    let config = get_config(&config_path_or_url).unwrap_or_else(|err| {
        println!("{}: {:?}", err, err);
        process::exit(1);
    });
    let zones = parse_config(&config);

    // convert current time to friend's timezone, and print:
    let local_time = get_local_time(&name, zones).unwrap_or_else(|err| {
        println!("{}: {:?}", err, err);
        process::exit(1);
    });
    println!("{}", local_time);
}
