pub use chrono::{DateTime, Utc, Local, NaiveDateTime, TimeZone};

pub fn datetime_from_input(input: &str) -> Option<DateTime<Utc>> {
    let ndt = NaiveDateTime::parse_from_str(input, "%F").ok()?;
    Local.from_local_datetime(&ndt).single().map(|dt| dt.to_utc())
}
