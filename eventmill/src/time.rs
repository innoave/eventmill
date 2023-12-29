#[cfg(feature = "chrono")]
pub type Date = chrono::NaiveDate;
#[cfg(feature = "chrono")]
pub type DateTime = chrono::DateTime<chrono::Utc>;

#[cfg(feature = "time")]
pub type Date = time::Date;
#[cfg(feature = "time")]
pub type DateTime = time::OffsetDateTime;
