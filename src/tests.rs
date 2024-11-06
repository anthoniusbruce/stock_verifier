#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use chrono::{TimeZone, Utc};

    use crate::validate_args;

    #[test]
    #[should_panic(expected = "not valid date")]
    fn validate_arguments_date_is_not_good_invalid_month() {
        // assign
        let html = PathBuf::from("testdata.html");
        let date = String::from("13-04-2024");
        let end_date = String::from("12-04-2024");

        // act
        let _actual = validate_args(&html, date, end_date);

        // assign
        assert!(false);
    }

    #[test]
    #[should_panic(expected = "not valid date")]
    fn validate_arguments_date_is_not_good_invalid_day() {
        // assign
        let html = PathBuf::from("testdata.html");
        let start_date = String::from("11-1-2024");
        let date = String::from("11-32-2024");

        // act
        let _actual = validate_args(&html, start_date, date);

        // assign
        assert!(false);
    }

    #[test]
    #[should_panic(expected = "not valid date")]
    fn validate_arguments_date_is_not_good_not_3_segments() {
        // assign
        let html = PathBuf::from("testdata.html");
        let date = String::from("11-04");
        let end_date = String::from("11-05-2024");

        // act
        let _actual = validate_args(&html, date, end_date);

        // assign
        assert!(false);
    }

    #[test]
    fn validate_arguments_date_is_good_dash() {
        // assign
        let html = PathBuf::from("testdata.html");
        let start_date = String::from("11-04-2024");
        let end_date = String::from("11-24-2024");
        let expected = (
            Utc.with_ymd_and_hms(2024, 11, 4, 0, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2024, 11, 24, 0, 0, 0).unwrap(),
        );

        // act
        let actual = validate_args(&html, start_date, end_date);

        // assign
        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_arguments_date_is_good_slash() {
        // assign
        let html = PathBuf::from("testdata.html");
        let start_date = String::from("11/04/2024");
        let end_date = String::from("11/24/2024");
        let expected = (
            Utc.with_ymd_and_hms(2024, 11, 4, 0, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2024, 11, 24, 0, 0, 0).unwrap(),
        );

        // act
        let actual = validate_args(&html, start_date, end_date);

        // assign
        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_arguments_html_exists() {
        // assign
        let html = PathBuf::from("testdata.html");
        let start_date = String::from("11/04/2024");
        let end_date = String::from("11/04/2024");

        // act
        validate_args(&html, start_date, end_date);

        // assign
        assert!(true);
    }

    #[test]
    #[should_panic(expected = "file does not exist")]
    fn validate_arguments_html_does_not_exist() {
        // assign
        let html = PathBuf::from("testdata.x");
        let start_date = String::from("11/04/2024");
        let end_date = String::from("11/24/2024");

        // act
        validate_args(&html, start_date, end_date);

        // assign
        assert!(false);
    }
}
