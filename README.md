# what-time

Print current time for friends in their time zone.

## Installation

1. Install the binary:

```
cargo install what-time
```

2. Create a config file as described below, and put it in `$HOME/.what-time`.

## Example:

```
>% what-time josh
Sat 05:32 AM
```

Time zones are stored in a config file where each line contains
a friend's name and their time zone, separate white space:

```
anna Pacific/Auckland
caleb America/New_York
jimmy Australia/Brisbane
joanne America/Los_Angeles
josh Australia/Brisbane
sebastian America/Phoenix
steve America/Chicago
thomi Pacific/Auckland
```

## Config File
The default config file location is `$HOME/.what-time` but you can
override with the `--config` flag:
```
>% what-time --config ~/.my-config josh
>% Sat 05:32 AM
```

See [List of tz database time zones](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones) for valid time zone strings.

The config can be a URL starting with /https?/, in which case the config will be fetched from that URL. For example:

```
>% what-time --config https://gist.githubusercontent.com/stevemolitor314/ecf5bd9b1c7c36a78c3ae1c29ff20a7b/raw/12d56e13ad3a3c191969ecac9034be0df2026c63/what-time.txt josh
>% Sat 05:32 AM
```

## Help

Pass the `--help` option to see the usage information:

```
>% what-time --help

Prints current time for friends in other time zones, in their time zone.

USAGE:
    what-time [OPTIONS] <NAME>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>    Friends timezones file, defaults to ~/.what-time. Can be a URL starting with /https?/, in
                           which case the config will be fetched from that URL.

ARGS:
    <NAME>    The name of the friend to report the current time for.
```
