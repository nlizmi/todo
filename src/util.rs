use std::{fmt::Display, io, path};
pub use chrono::{DateTime, Utc, Local, NaiveDateTime, TimeZone};

pub fn numbered_list_as_string<I>(v: impl Iterator<Item = I>) -> String where I: Display {
    let iter = v.enumerate().map(|(i, x)| (i, x.to_string()));
    let mut s = iter.map(|(i, s)| format!("{}.\t{}", i, s)).fold(String::new(), |a, b| a + &b + "\n");
    s.pop();
    s
}

pub fn datetime_from_input(input: &str) -> Option<DateTime<Local>> {
    let ndt = NaiveDateTime::parse_from_str(input, "%F %-H:%M:%S").ok()?;
    Local.from_local_datetime(&ndt).single()
}

pub fn read_input(buffer: &mut String) -> io::Result<()> {
    buffer.clear();
    io::stdin().read_line(buffer)?;
    *buffer = buffer.trim().to_owned();
    Ok(())
}

pub fn invalid_input_error(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, msg)
}

pub fn data_path() -> io::Result<path::PathBuf> {
    dirs::data_dir().map(|dir| dir.join("nlizmi-todo").join("data.cbor")).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidFilename, "couldn't find the data directory"))
}
