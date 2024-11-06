use std::{
    fs,
    path::{Path, PathBuf},
};

use html_editor::parse;
use structopt::StructOpt;
use time::{Date, Month, OffsetDateTime, Time};

mod tests;

#[derive(StructOpt)]
#[structopt(
    name = "stock_verifier",
    about = "modifies html produced by stock_simulator to add actual results to compare historical runs of stock_simulator. Dates are acceptable as MM/DD/YYY or MM-DD-YYYY."
)]
struct Opt {
    #[structopt(parse(from_os_str), required(true))]
    html_file: PathBuf,
    #[structopt(required(true))]
    start_date: String,
    #[structopt(required(true))]
    end_date: String,
}

fn main() {
    let opt = Opt::from_args_safe();

    match opt {
        Ok(args) => {
            let dates = validate_args(&args.html_file, args.start_date, args.end_date);
            modify_html(args.html_file, dates.0, dates.1);
        }
        Err(e) => println!("{e}"),
    }
}

fn modify_html(path: PathBuf, start_date: OffsetDateTime, end_date: OffsetDateTime) {
    let result: Result<String, std::io::Error>;
    {
        result = fs::read_to_string(path);
    }
    match result {
        Ok(s) => {
            let mut dom = parse(&s).expect("html could not be parsed");
        }
        Err(e) => panic!("{e}"),
    }
}

fn validate_args(
    html: &PathBuf,
    start_date_string: String,
    end_date_string: String,
) -> (OffsetDateTime, OffsetDateTime) {
    let file_exists = Path::exists(html);
    if !file_exists {
        panic!("file does not exist");
    }

    (
        validate_date(start_date_string),
        validate_date(end_date_string),
    )
}

fn validate_date(date_string: String) -> OffsetDateTime {
    let mut parts: Vec<&str> = date_string.split('-').collect();
    if parts.len() != 3 {
        parts = date_string.split('/').collect();
    }
    if parts.len() != 3 {
        panic!("not valid date");
    }
    let month: u8 = parts[0].parse().expect("not valid date");
    let day: u8 = parts[1].parse().expect("not valid date");
    let year: u16 = parts[2].parse().expect("not valid date");

    let mnth = Month::try_from(month).expect("not valid date");
    let date = OffsetDateTime::new_utc(
        Date::from_calendar_date(year.into(), mnth, day).expect("not valid date"),
        Time::from_hms_nano(0, 0, 0, 0).expect("not valid date"),
    );

    date
}
