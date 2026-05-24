use std::{fmt::Display, rc::Rc};
use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag_no_case, character::complete::multispace1, combinator::{map, map_res}
};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, FromRepr};
use serde_derive::{Serialize, Deserialize};

pub mod color_util;
pub use color_util::*;

pub mod chrono_util;
pub use chrono_util::*;

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
        data.groups.iter().find(|c| name.to_lowercase() == c.name.to_lowercase()).map(|c| Rc::clone(c))
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Todo {
    pub desc: String,
    pub category: Option<Rc<Group>>,
    pub due: Option<DateTime<Utc>>,
    pub urgency: Urgency,
    pub created: DateTime<Utc>,
}
impl Todo {
    pub fn from(desc: String, category: Option<Rc<Group>>, due: Option<DateTime<Utc>>, urgency: Urgency) -> Self {
        Self { desc, category, due, urgency, created: Local::now().to_utc() }
    }
}
impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let category = match &self.category {
            Some(c) => &format!("{}: ", c),
            None => "",
        };
        let colored_desc = Color::Default.bold().paint(&self.desc);
        let by_due_date = match self.due {
            Some(dt) => &format!(" by {}", dt),
            None => "",
        };
        write!(f, "{}do {}{} (urgency {})", category, colored_desc, by_due_date, self.urgency)
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
        // regex: "(q|quit|(category|todo)\s+(add|edit))"
        alt(
            (
                map_res(
                    (
                        alt(
                            (
                                tag_no_case("group"),
                                tag_no_case("todo"),
                            )
                        ),
                        multispace1,
                        alt(
                            (
                                tag_no_case("add"),
                                tag_no_case("edit"),
                                tag_no_case("list"),
                            )
                        )
                    ),
                    |(entity, _, action)| match (entity, action) {
                        ("group", "add") => Ok(Self::GroupAdd),
                        ("group", "edit") => Ok(Self::GroupEdit),
                        ("group", "list") => Ok(Self::GroupList),
                        ("todo", "add") => Ok(Self::TodoAdd),
                        ("todo", "edit") => Ok(Self::TodoEdit),
                        ("todo", "list") => Ok(Self::TodoList),
                        _ => Err(()),
                    }
                ),
                map(
                    alt(
                        (
                            tag_no_case("quit"),
                            tag_no_case("q"),
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