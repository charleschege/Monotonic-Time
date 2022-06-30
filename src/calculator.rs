use crate::{DateTimeFormat, MONTHS_LEAP_YEAR, MONTHS_NON_LEAP_YEAR};
use core::fmt;

/// `DateTime` struct handles all operations for time and date conversion
/// from seconds
#[derive(Debug)]
pub struct DateTime {
    year: u32,
    month: u8,
    day: u16,
    hour: u8,
    minutes: u8,
    seconds: u8,
    datetime_format: DateTimeFormat,
}

impl DateTime {
    /// Create a new [DateTime]
    pub fn new() -> Self {
        DateTime {
            year: u32::default(),
            month: u8::default(),
            day: u16::default(),
            hour: u8::default(),
            minutes: u8::default(),
            seconds: u8::default(),
            datetime_format: DateTimeFormat::default(),
        }
    }

    fn change_format(&mut self, format: DateTimeFormat) -> &mut Self {
        self.datetime_format = format;

        self
    }

    /// Convert the seconds to TAI time using the offset provided
    /// by  utc_behind_by`
    pub fn to_taitime(&mut self, seconds: u64, utc_behind_by: u64) -> &mut Self {
        self.change_format(DateTimeFormat::Tai);
        self.to_datetime(seconds + utc_behind_by)
    }

    /// Convert the seconds provided to UTC
    pub fn to_datetime(&mut self, seconds: u64) -> &mut Self {
        let mut year = 1970;

        let dayclock = seconds % 86400;
        let mut daynumber = seconds / 86400;

        self.seconds = (dayclock % 60) as u8;
        self.minutes = ((dayclock % 3600) / 60) as u8;
        self.hour = (dayclock / 3600) as u8;

        loop {
            let yearsize = self.year_days(year);

            if daynumber >= yearsize {
                daynumber -= yearsize;

                year += 1;
            } else {
                break;
            }
        }

        self.year = year as u32;

        let mut month = 0;

        let month_array = if self.is_leap_year(self.year) {
            MONTHS_LEAP_YEAR
        } else {
            MONTHS_NON_LEAP_YEAR
        };

        while daynumber >= month_array[month] as u64 {
            daynumber -= month_array[month] as u64;

            month += 1;
        }

        self.month = month as u8 + 1;
        self.day = daynumber as u16 + 1;

        self
    }

    /* TODO
    /// Converts `UTC` or `TAI` time to local timezone using `offset` provided
    pub fn to_timezone(&mut self, offset: i8) -> &mut Self {
        let mut hour = self.hour as i8;
        hour += offset;

        self.hour = hour as u8;

        self
    } */

    fn is_leap_year(&self, value: u32) -> bool {
        value % 4 == 0 && (value % 100 != 0 || value % 400 == 0)
    }

    fn year_days(&self, value: u32) -> u64 {
        if self.is_leap_year(value) {
            366
        } else {
            365
        }
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02} {}",
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minutes,
            self.seconds,
            self.datetime_format
        )
    }
}
