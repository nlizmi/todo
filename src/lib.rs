use std::{collections::BTreeSet, fmt::Display, fs, io, rc::Rc};
use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag_no_case, character::complete::multispace1, combinator::{map, map_res}
};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, FromRepr};
use serde_derive::{Serialize, Deserialize};

pub mod util;
use util::*;
use ansiterm::Colour::*;

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Color(pub ansiterm::Colour);
impl Color {
    pub fn from_input(input: &str) -> Option<Self> {
        let variants = Self::variants();
        match input.parse::<usize>() {
            Ok(n) => variants.get(n).copied(),
            Err(_) => variants.iter().find(|c| input.to_lowercase() == c.as_str().to_lowercase()).copied()
        }
    }
    pub fn variants() -> Vec<Self> {
        static VARIANTS: [Color; 17] = [
            Color(Black),
            Color(Red),
            Color(Green),
            Color(Yellow),
            Color(Blue),
            Color(Purple),
            Color(Cyan),
            Color(White),
            Color(DarkGray),
            Color(BrightRed),
            Color(BrightGreen),
            Color(BrightYellow),
            Color(BrightBlue),
            Color(BrightPurple),
            Color(BrightCyan),
            Color(BrightGray),
            Color(Default)
        ];
        VARIANTS.to_vec()
    }
    pub fn as_str(&self) -> &str {
        match self.0 {
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
        }
    }
}
impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.paint(self.as_str()))
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
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
            Ok(n) => data.groups.iter().nth(n),
            Err(_) => data.groups.iter().find(|c| name.to_lowercase() == c.name.to_lowercase())
        }.map(|c| Rc::clone(c))
    }
}
impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.color.0.paint(&self.name))
    }
}
impl Eq for Group {}
impl Ord for Group {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}
impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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
            Self::LongTerm => Blue,
            Self::Low => Green,
            Self::Medium => Yellow,
            Self::High => Red,
        };
        write!(f, "{}", color.paint(self.as_str()))
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, EnumIter, FromRepr)]
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
            Self::InProgress => White,
            Self::Done => BrightGray,
            Self::Abandoned => DarkGray,
        };
        write!(f, "{}", color.paint(self.as_str()))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Serialize, Deserialize)]
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
        let colored_desc = White.bold().paint(&self.desc);
        let due_date = match self.due {
            Some(dt) => &format!(" due {}", BrightYellow.paint(dt.format("%a %b %-d, %Y at %-I:%M:%S %P").to_string())),
            None => "",
        };
        write!(f, "{}{}{} ({} urgency) is {}", group, colored_desc, due_date, self.urgency, self.progress)
    }
}
impl Ord for Todo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.due.cmp(&other.due).then(self.urgency.cmp(&other.urgency)).then(self.desc.cmp(&other.desc))
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoData {
    pub groups: BTreeSet<Rc<Group>>,
    pub todos: BTreeSet<Todo>,
}
impl TodoData {
    pub fn new() -> Self {
        Self {
            groups: BTreeSet::new(),
            todos: BTreeSet::new(),
        }
    }
    pub fn save(&self) -> io::Result<()> {
        let path = util::data_path()?;
        fs::create_dir_all(path.parent().unwrap())?;
        let file = fs::File::create(path)?;
        serde_cbor::to_writer(file, self).map_err(|_| io::Error::new(io::ErrorKind::Other, "couldn't save to the data file"))
    }
    pub fn load() -> Self {
        let load = || -> io::Result<Self> {
            let path = util::data_path()?;
            let file = fs::File::open(path)?;
            serde_cbor::from_reader(file).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        };

        load().unwrap_or_else(|_| Self::new())
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