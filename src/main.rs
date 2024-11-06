use std::path::{Path, PathBuf};

use chrono::{offset::LocalResult, DateTime, TimeZone, Utc};

mod tests;

fn main() {
    println!("Hello, world!");
}

fn validate_args(
    html: &PathBuf,
    start_date_string: String,
    end_date_string: String,
) -> (DateTime<Utc>, DateTime<Utc>) {
    let file_exists = Path::exists(html);
    if !file_exists {
        panic!("file does not exist");
    }

    (
        validate_date(start_date_string),
        validate_date(end_date_string),
    )
}

fn validate_date(date_string: String) -> DateTime<Utc> {
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

    let date: DateTime<Utc>;
    let date_result: LocalResult<DateTime<Utc>> =
        Utc.with_ymd_and_hms(year as i32, month as u32, day as u32, 0, 0, 0);
    match date_result {
        LocalResult::Single(dt) => {
            date = dt;
        }
        LocalResult::Ambiguous(dt, _) => {
            date = dt;
        }
        LocalResult::None => {
            panic!("not valid date");
        }
    }

    date
}
