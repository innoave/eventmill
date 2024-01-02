#[cfg(all(feature = "time", feature = "chrono"))]
use chrono as _;

#[cfg(all(feature = "chrono", not(feature = "time")))]
pub type Date = chrono::NaiveDate;
#[cfg(all(feature = "chrono", not(feature = "time")))]
pub type DateTime = chrono::DateTime<chrono::Utc>;

#[cfg(feature = "time")]
pub type Date = time::Date;
#[cfg(feature = "time")]
pub type DateTime = time::OffsetDateTime;
