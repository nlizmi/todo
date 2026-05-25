use std::{cell::RefCell, fmt::Display, fs, io, rc::Rc};
use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag_no_case, character::complete::multispace1, combinator::{map, map_res}
};
use strum_macros::{EnumIter, FromRepr};
use serde_derive::{Serialize, Deserialize};

pub mod util;
use util::*;
use ansiterm::Colour::*;

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Color(pub ansiterm::Colour);
impl Color {
    #[allow(non_upper_case_globals)]
    pub const variants: [Color; 17] =[
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
    pub fn as_str(&self) -> &str {
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
            Self::InProgress => BrightBlue.normal(),
            Self::Done => BrightGreen.normal(),
            Self::Abandoned => Red.dimmed(),
        };
        write!(f, "{}", color.paint(self.as_str()))
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd)]
pub struct TodoItem {
    pub desc: String,
    pub group: Option<Rc<RefCell<Group>>>,
    pub due: Option<DateTime<Local>>,
    pub urgency: Urgency,
    pub progress: Progress,
    pub created: DateTime<Local>,
}
impl TodoItem {
    pub fn from(desc: String, group: Option<Rc<RefCell<Group>>>, due: Option<DateTime<Local>>, urgency: Urgency) -> Self {
        Self { desc, group, due, urgency, progress: Progress::InProgress, created: Local::now() }
    }
}
impl Display for TodoItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let group = match &self.group {
            Some(g) => &format!("[{}] ", g.borrow()),
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
impl Ord for TodoItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.due.cmp(&other.due).then(self.urgency.cmp(&other.urgency)).then(self.desc.cmp(&other.desc))
    }
}

pub struct TodoData {
    pub groups: Vec<Rc<RefCell<Group>>>,
    pub todos: Vec<TodoItem>,
}
impl TodoData {
    pub fn new() -> Self {
        Self {
            groups: Vec::new(),
            todos: Vec::new(),
        }
    }
    pub fn save(&self) -> io::Result<()> {
        let path = util::data_path()?;
        fs::create_dir_all(path.parent().unwrap())?;
        let file = fs::File::create(path)?;
        let saved = TodoDataSaved::from_unsaved(self);
        serde_cbor::to_writer(file, &saved).map_err(|_| io::Error::new(io::ErrorKind::Other, "couldn't save to the data file"))
    }
    pub fn load() -> Self {
        let load = || -> io::Result<Self> {
            let path = util::data_path()?;
            let file = fs::File::open(path)?;
            serde_cbor::from_reader::<TodoDataSaved, _>(file).map(|saved| saved.to_unsaved()).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        };

        load().unwrap_or_else(|_| Self::new())
    }
}

#[derive(Serialize, Deserialize)]
/// necessary because `Rc`s don't serialize well :(
struct TodoItemSaved {
    desc: String,
    group_index: Option<usize>,
    due: Option<DateTime<Local>>,
    urgency: Urgency,
    progress: Progress,
    created: DateTime<Local>,
}

#[derive(Serialize, Deserialize)]
/// necessary because `Rc`s don't serialize well :(
struct TodoDataSaved {
    groups: Vec<Group>,
    todos: Vec<TodoItemSaved>,
}
impl TodoDataSaved {
    fn from_unsaved(data: &TodoData) -> TodoDataSaved {
        let groups = data.groups.iter().map(|g| g.borrow().clone()).collect();
        let todos = data.todos.iter().map(|t| TodoItemSaved {
            desc: t.desc.clone(),
            group_index: t.group.as_ref().and_then(|tg| data.groups.iter().position(|g| Rc::ptr_eq(tg, g))),
            due: t.due,
            urgency: t.urgency.clone(),
            progress: t.progress.clone(),
            created: t.created,
        }).collect();
        TodoDataSaved { groups, todos }
    }
    fn to_unsaved(self) -> TodoData {
        let groups: Vec<Rc<RefCell<Group>>> = self.groups.into_iter().map(|g| Rc::new(RefCell::new(g))).collect();
        let todos = self.todos.into_iter().map(|t| TodoItem {
            desc: t.desc,
            group: t.group_index.and_then(|i| groups.get(i).cloned()),
            due: t.due,
            urgency: t.urgency,
            progress: t.progress,
            created: t.created,
        }).collect();
        TodoData { groups, todos }
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