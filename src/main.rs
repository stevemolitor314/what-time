use what_time::*;

fn main() {
    // get command line args:
    let CmdLineArgs { config_path, name } = get_cmd_line_args();

    // read and parse config file:
    let config = get_config(&config_path);
    let zones = parse_config(&config);

    // convert current time to friend's timezone, and print:
    let local_time = get_local_time(&name, zones);
    println!("{}", local_time);
}
