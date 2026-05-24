pub use chrono::{DateTime, Utc, Local, NaiveDateTime, TimeZone};
pub use ansiterm::Color;
use ansiterm::Color::*;

pub fn datetime_from_input(input: &str) -> Option<DateTime<Local>> {
    let ndt = NaiveDateTime::parse_from_str(input, "%F %-H:%M:%S").ok()?;
    Local.from_local_datetime(&ndt).single()
}

pub fn color_from_input(input: &str) -> Option<Color> {
    let variants = color_variants();
    match input.parse::<usize>() {
        Ok(n) => variants.get(n).copied(),
        Err(_) => variants.iter().find(|c| input.to_lowercase() == color_to_string(c).to_lowercase()).copied()
    }
}
pub fn color_to_string(color: &Color) -> String {
    match color {
        Black => "black",
        Red => "red",
        Green => "green",
        Yellow => "yellow",
        Blue => "blue",
        Purple => "purple",
        Cyan => "cyan",
        White => "white",
        DarkGray => "dark-gray",
        BrightRed => "bright-red",
        BrightGreen => "bright-green",
        BrightYellow => "bright-yellow",
        BrightBlue => "bright-blue",
        BrightPurple => "bright-purple",
        BrightCyan => "bright-cyan",
        BrightGray => "bright-gray",
        _ => "default",
    }.to_owned()
}
pub fn color_variants() -> Vec<Color> {
    static VARIANTS: [Color; 17] = [Black, Red, Green, Yellow, Blue, Purple, Cyan, White, DarkGray, BrightRed, BrightGreen, BrightYellow, BrightBlue, BrightPurple, BrightCyan, BrightGray, Default];
    VARIANTS.to_vec()
}
pub fn color_options() -> impl Iterator<Item = (usize, String)> {
    color_variants().into_iter().enumerate().map(|(i, c)| (i, color_to_string(&c)))
}
