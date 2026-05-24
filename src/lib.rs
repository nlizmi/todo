use std::{fmt::Display, rc::Rc};
use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag_no_case, character::complete::multispace1, combinator::{map, map_res}
};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, FromRepr};
use serde_derive::{Serialize, Deserialize};

pub mod util;
pub use util::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    pub color: Color,
}
impl Group {
    pub fn from(name: String, color: Color) -> Self {
        Self { name: name.to_owned(), color }
    }
    pub fn from_data(data: &TodoData, name: &str) -> Option<Rc<Self>> {
        match name.parse::<usize>() {
            Ok(n) => data.groups.get(n),
            Err(_) => data.groups.iter().find(|c| name.to_lowercase() == c.name.to_lowercase())
        }.map(|c| Rc::clone(c))
    }
    pub fn options(data: &TodoData) -> impl Iterator<Item = (usize, String)> {
        data.groups.iter().enumerate().map(|(i, c)| (i, c.to_string()))
    }
}
impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.color.paint(&self.name))
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, EnumIter, FromRepr)]
pub enum Urgency {
    LongTerm,
    Low,
    Medium,
    High,
}
impl Urgency {
    pub fn from(input: &str) -> Option<Self> {
        match input.parse::<usize>() {
            Ok(n) => Self::from_repr(n),
            Err(_) => Self::iter().find(|u| input.to_lowercase() == u.as_str().to_lowercase())
        }
    }
    pub fn options() -> impl Iterator<Item = (usize, String)> {
        Self::iter().enumerate().map(|(i, u)| (i, u.to_string()))
    }
    pub fn as_str(&self) -> &str {
        match self {
            Self::LongTerm => "long-term",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }
}
impl Display for Urgency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = match self {
            Self::LongTerm => Color::Blue,
            Self::Low => Color::Green,
            Self::Medium => Color::Yellow,
            Self::High => Color::Red,
        };
        write!(f, "{}", color.paint(self.as_str()))
    }
}

#[derive(Clone, Serialize, Deserialize, EnumIter, FromRepr)]
pub enum Progress {
    InProgress,
    Done,
    Abandoned,
}
impl Progress {
    pub fn from(input: &str) -> Option<Self> {
        match input.parse::<usize>() {
            Ok(n) => Self::from_repr(n),
            Err(_) => Self::iter().find(|p| input.to_lowercase() == p.as_str().to_lowercase())
        }
    }
    pub fn options() -> impl Iterator<Item = (usize, String)> {
        Self::iter().enumerate().map(|(i, u)| (i, u.to_string()))
    }
    fn as_str(&self) -> &str {
        match self {
            Self::InProgress => "in progress",
            Self::Done => "done",
            Self::Abandoned => "abandoned",
        }
    }
}
impl Display for Progress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = match self {
            Self::InProgress => Color::White,
            Self::Done => Color::BrightGray,
            Self::Abandoned => Color::DarkGray,
        };
        write!(f, "{}", color.paint(self.as_str()))
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Todo {
    pub desc: String,
    pub group: Option<Rc<Group>>,
    pub due: Option<DateTime<Local>>,
    pub urgency: Urgency,
    pub progress: Progress,
    pub created: DateTime<Local>,
}
impl Todo {
    pub fn from(desc: String, group: Option<Rc<Group>>, due: Option<DateTime<Local>>, urgency: Urgency) -> Self {
        Self { desc, group, due, urgency, progress: Progress::InProgress, created: Local::now() }
    }
}
impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let group = match &self.group {
            Some(c) => &format!("[{}] ", c),
            None => "",
        };
        let colored_desc = Color::White.bold().paint(&self.desc);
        let due_date = match self.due {
            Some(dt) => &format!(" due {}", Color::White.paint(dt.format("%a %-d %b %Y at %-I:%M:%S %P").to_string())),
            None => "",
        };
        write!(f, "{}{}{} ({} urgency) is {}", group, colored_desc, due_date, self.urgency, self.progress)
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoData {
    pub groups: Vec<Rc<Group>>,
    pub todos: Vec<Todo>,
}
impl TodoData {
    pub fn new() -> Self {
        Self {
            groups: vec![],
            todos: vec![]
        }
    }
}

pub enum Command {
    GroupAdd,
    GroupEdit,
    GroupList,
    TodoAdd,
    TodoEdit,
    TodoList,
    Help,
    Quit,
}
impl Command {
    pub fn parse_from(input: &str) -> IResult<&str, Self> {
        alt(
            (
                map_res(
                    (
                        alt(
                            (
                                tag_no_case("group"),
                                tag_no_case("g"),
                                tag_no_case("todo"),
                                tag_no_case("t"),
                            )
                        ),
                        multispace1,
                        alt(
                            (
                                tag_no_case("add"),
                                tag_no_case("edit"),
                                tag_no_case("list"),
                                tag_no_case("ls"),
                            )
                        )
                    ),
                    |(entity, _, action)| match (entity, action) {
                        ("group" | "g", "add") => Ok(Self::GroupAdd),
                        ("group" | "g", "edit") => Ok(Self::GroupEdit),
                        ("group" | "g", "list" | "ls") => Ok(Self::GroupList),
                        ("todo" | "t", "add") => Ok(Self::TodoAdd),
                        ("todo" | "t", "edit") => Ok(Self::TodoEdit),
                        ("todo" | "t", "list" | "ls") => Ok(Self::TodoList),
                        _ => Err(()),
                    }
                ),
                map(
                    alt(
                        (
                            tag_no_case("quit"),
                            tag_no_case("q"),
                            tag_no_case("exit"),
                        )
                    ),
                    |_| Self::Quit
                ),
                map(
                    alt(
                        (
                            tag_no_case("help"),
                            tag_no_case("h"),
                        )
                    ),
                    |_| Self::Help
                ),
            )
        ).parse(input)
    }
}