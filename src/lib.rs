use chrono::{DateTime, Utc, Local, NaiveDateTime, TimeZone};
use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag_no_case, character::complete::multispace1, combinator::map
};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, FromRepr};
use ansiterm::Color;

pub trait ColorUtilities where Self: Sized {
    fn from_input(input: &str) -> Option<Self>;
    fn to_string(&self) -> String;
    fn variants() -> Vec<Self>;
    fn options() -> impl Iterator<Item = (usize, String)>;
}
impl ColorUtilities for Color {
    fn from_input(input: &str) -> Option<Self> {
        Self::variants().iter().find(|c| input.to_lowercase() == c.to_string().to_lowercase()).copied()
    }
    fn to_string(&self) -> String {
        use Color::*;
        match self {
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
    fn variants() -> Vec<Color> {
        use Color::*;
        static VARIANTS: [Color; 17] = [Black, Red, Green, Yellow, Blue, Purple, Cyan, White, DarkGray, BrightRed, BrightGreen, BrightYellow, BrightBlue, BrightPurple, BrightCyan, BrightGray, Default];
        VARIANTS.to_vec()
    }
    fn options() -> impl Iterator<Item = (usize, String)> {
        Self::variants().into_iter().enumerate().map(|(i, v)| (i, v.to_string()))
    }
}

pub trait DateTimeUtilities {
    fn from_input(input: &str) -> Option<Self> where Self: Sized;
}
impl DateTimeUtilities for DateTime<Utc> {
    fn from_input(input: &str) -> Option<Self> {
        let ndt = NaiveDateTime::parse_from_str(input, "%F").ok()?;
        Local.from_local_datetime(&ndt).single().map(|dt| dt.to_utc())
    }
}

pub struct Category {
    pub name: String,
    pub color: Color,
}
impl Category {
    pub fn from(name: String, color: Color) -> Self {
        Self { name: name.to_owned(), color }
    }
    pub fn from_data<'a>(data: &'a TodoData, name: &str) -> Option<&'a Self> {
        data.categories.iter().find(|c| name.to_lowercase() == c.name.to_lowercase())
    }
    pub fn options(data: &TodoData) -> impl Iterator<Item = (usize, String)> {
        data.categories.iter().enumerate().map(|(i, c)| (i, c.color.paint(&c.name).to_string()))
    }
}

#[derive(EnumIter, FromRepr)]
pub enum Urgency {
    Low,
    Medium,
    High,
    LongTerm,
}
impl Urgency {
    pub fn from(s: &str) -> Option<Self> {
        match s.parse::<usize>() {
            Ok(n) => Self::from_repr(n),
            Err(_) => Self::iter().find(|u| s.to_lowercase() == u.to_string().to_lowercase())
        }
    }
    pub fn options() -> impl Iterator<Item = (usize, String)> {
        Self::iter().enumerate().map(|(i, u)| (i, u.to_string()))
    }
}
impl ToString for Urgency {
    fn to_string(&self) -> String {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::LongTerm => "long-term",
        }.to_owned()
    }
}

pub struct Todo<'a> {
    pub desc: String,
    pub category: &'a Category,
    pub due: Option<DateTime<Utc>>,
    pub urgency: Urgency,
    pub created: DateTime<Utc>,
}
impl <'a> Todo<'a> {
    pub fn from(desc: String, category: &'a Category, due: Option<DateTime<Utc>>, urgency: Urgency) -> Self {
        Self { desc, category, due, urgency, created: Local::now().to_utc() }
    }
}

pub struct TodoData<'a> {
    pub categories: Vec<Category>,
    pub todos: Vec<Todo<'a>>,
}
impl TodoData<'_> {
    pub fn new() -> Self {
        Self {
            categories: vec![],
            todos: vec![]
        }
    }
}

pub enum Command {
    CategoryAdd,
    CategoryEdit,
    TodoAdd,
    TodoEdit,
    Quit,
}
impl Command {
    pub fn parse_from(input: &str) -> IResult<&str, Self> {
        // regex: "(q|quit|(category|todo)\s+(add|edit))"
        alt(
            (
                map(
                    (
                        alt(
                            (
                                tag_no_case("category"),
                                tag_no_case("todo"),
                            )
                        ),
                        multispace1,
                        alt(
                            (
                                tag_no_case("add"),
                                tag_no_case("edit"),
                            )
                        )
                    ),
                    |(entity, _, action)| match (entity, action) {
                        ("category", "add") => Self::CategoryAdd,
                        ("category", "edit") => Self::CategoryEdit,
                        ("todo", "add") => Self::TodoAdd,
                        ("todo", "edit") => Self::TodoEdit,
                        _ => unreachable!(),
                    }
                ),
                map(
                    alt(
                        (
                            tag_no_case("q"),
                            tag_no_case("quit"),
                        )
                    ),
                    |_| Self::Quit
                ),
            )
        ).parse(input)
    }
}