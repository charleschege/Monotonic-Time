use core::fmt;

/// Date and time formats currently supports `UTC` and `TAI`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum DateTimeFormat {
    /// The `Coordinated Universal Time` time format
    Utc,
    /// The ` Temps Atomique International` time format
    Tai,
}

impl Default for DateTimeFormat {
    fn default() -> Self {
        DateTimeFormat::Utc
    }
}
impl fmt::Display for DateTimeFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Utc => "UTC",
                Self::Tai => "TAI",
            }
        )
    }
}
