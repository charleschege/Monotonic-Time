#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![doc = include_str!("../README.md")]

mod calculator;
pub use calculator::*;

mod contants;
pub use contants::*;

mod datetime_formats;
pub use datetime_formats::*;

#[cfg(test)]
mod sanity_tests {
    use crate::DateTime;

    #[test]
    fn test_utc() {
        let now = 1656603896;

        let mut datetime = DateTime::new();
        datetime.to_datetime(now);
        assert_eq!("2022-06-30 15:44:56 UTC", &format!("{}", datetime));
    }

    #[test]
    fn test_tai() {
        let now = 1656603896;

        let mut datetime = DateTime::new();
        datetime.to_taitime(now, 37);
        assert_eq!("2022-06-30 15:45:33 TAI", &format!("{}", datetime));
    }
}
