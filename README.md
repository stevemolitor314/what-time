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
    -c, --config <FILE>    Friends timezones file, defaults to ~/.what-time

ARGS:
    <NAME>    The name of the friend to report the current time for.
```
