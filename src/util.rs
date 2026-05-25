use std::{fmt::Display, io, path};
pub use chrono::{DateTime, Utc, Local, NaiveDateTime, TimeZone};

pub fn get_choice_index<'a, T>(options: &'a [T], f: impl Fn(&T) -> String, input: &str) -> io::Result<usize> {
    match input.parse::<usize>() {
        Ok(i) if i < options.len() => Ok(i),
        Ok(i) => Err(invalid_input_error(&format!("index out of bounds: {}", i))),
        Err(_) => options.iter().position(|option| input.to_lowercase() == f(option).to_lowercase()).ok_or_else(|| invalid_input_error(&format!("invalid choice: {}", input)))
    }
}

pub fn numbered_list_as_string<I>(v: impl Iterator<Item = I>) -> String where I: Display {
    let iter = v.enumerate().map(|(i, x)| (i, x.to_string()));
    let mut s = iter.map(|(i, s)| format!("{}.\t{}", i, s)).fold(String::new(), |a, b| a + &b + "\n");
    s.pop();
    s
}

pub fn datetime_from_input(input: &str) -> io::Result<DateTime<Local>> {
    let ndt = NaiveDateTime::parse_from_str(input, "%F %-H:%M:%S").map_err(|_| invalid_input_error(&format!("invalid date & time: {}", input)))?;
    Local.from_local_datetime(&ndt).single().ok_or_else(|| invalid_input_error(&format!("this date & time is invalid because it falls on a daylight savings time border: {}", input)))
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
