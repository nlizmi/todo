use chrono::{NaiveDateTime, DateTime, Local};
use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag_no_case, character::complete::multispace1, combinator::map
};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, FromRepr};
use ansiterm::Color;

trait Options {
    
}

pub trait FromInput {
    fn from(input: &str) -> Self;
}
impl FromInput for DateTime<Local> {
    fn from(input: &str) -> Self {
        
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
        data.categories.iter().find(|c| c.name == name)
    }
    pub fn options(data: &TodoData) -> Vec<(usize, String)> {
        data.categories.iter().enumerate().map(|(i, c)| (i, c.color.paint(&c.name).to_string())).collect()
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
            Err(_) => Self::iter().find(|u| &u.to_string() == s)
        }
    }
    pub fn options() -> Vec<(usize, String)> {
        Self::iter().enumerate().map(|(i, u)| (i, u.to_string())).collect()
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
    pub due: Option<DateTime<Local>>,
    pub urgency: Urgency,
    pub created: DateTime<Local>,
}
impl <'a> Todo<'a> {
    pub fn from(desc: String, category: &'a Category, due: Option<DateTime<Local>>, urgency: Urgency) -> Self {
        Self { desc, category, due, urgency, created: Local::now() }
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