extern crate clap;
extern crate chrono;
extern crate ansi_term;
extern crate term_grid;

use clap::{Arg, App, ArgMatches, SubCommand, AppSettings};
use clap::crate_version;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc, TimeZone, Duration, Local};
use std::str::FromStr;
use ansi_term::Colour::{Red, Green, Yellow, Blue, Cyan, Purple, RGB};
use ansi_term::{ANSIString, ANSIStrings, Style};
use term_grid::{Grid, GridOptions, Filling, Direction, Cell};
use chrono::format::Numeric::Timestamp;
use std::ops::Add;
use chrono::format::Fixed::TimezoneOffset;


fn validate_i64(v: String) -> Result<(), String> {
    return match i64::from_str(v.as_str()) {
        Ok(_) => Ok(()),
        Err(_) => Err("not a valid integer".to_string())
    }
}

fn print_grid(seconds: i64) {
    let mut grid = Grid::new(GridOptions {
       filling: Filling::Spaces(4),
       direction: Direction:: LeftToRight,
    });

    let duration = Duration::seconds(seconds);

    grid.add(Cell {contents: format!("{}", Red.paint("Seconds")), width: 7 });
    grid.add(seconds.to_string().into());

    grid.add(Cell {contents: format!("{}", Blue.paint("Datetime")), width: 8});
    let datetime = Utc.timestamp(seconds, 0);
    grid.add(datetime.to_string().into());

    grid.add(Cell {contents: format!("{}", Yellow.paint("Human")), width: 5});
    let offset = Local::now().offset().clone();
    grid.add(datetime.with_timezone(&offset).to_rfc2822().into());
    print!("{}", grid.fit_into_columns(2));
}

fn main() {
    let matches = App::new("tt")
        .setting(AppSettings::AllowLeadingHyphen)
        .version(crate_version!())
        .about("time transform utility v0.1.0")
        .arg(Arg::with_name("seconds")
            .validator(validate_i64)
            .takes_value(true)
            .index(1)
            .help("integer number for unix timestamp")
        )
        .subcommand(SubCommand::with_name("add")
            .setting(AppSettings::AllowLeadingHyphen)
            .about("Use to add to a time")
            .arg(
                Arg::with_name("duration")
                    .validator(validate_i64)
                    .required(true)
                    .takes_value(true)
                    .index(1)
                    .help("integer number for how much to add by")
            )
        )
        .get_matches();

    let mut seconds;
    let now: DateTime<Local> = Local::now();
    seconds = now.timestamp();
    if let Some(seconds_str) = matches.value_of("seconds") {
        seconds = i64::from_str(seconds_str).unwrap();
    };

    if let Some(sub_matches) = matches.subcommand_matches("add") {
       let duration = i64::from_str(sub_matches.value_of("duration").unwrap()).unwrap();
       let seconds = seconds + duration;
       print_grid(seconds);
    } else {
        print_grid(seconds);
    }
}
