# Time Transform

Time Transform is a CLI for working with unix timestamps.

## Installation

binaries can be found in [releases](https://github.com/RohanArepally/tt/tree/master/releases).

#### Mac OSX
```bash
curl -L https://github.com/RohanArepally/tt/raw/master/releases/tt-0.1.0-x86_64-apple-darwin.tar.gz | tar -xv && sudo mv tt /usr/local/bin
```

## Usage
```$xslt
tt 0.1.0
time transform utility

USAGE:
    tt [seconds] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <seconds>    integer number for unix timestamp

SUBCOMMANDS:
    add     Use to add to a time
    help    Prints this message or the help of the given subcommand(s)
```