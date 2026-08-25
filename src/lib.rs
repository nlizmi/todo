use std::{cell::RefCell, cmp, fmt::{self, Display}, fs, io, rc::Rc};
use nom::{IResult, Parser, branch::alt, bytes::complete::tag_no_case, character::complete::multispace1, combinator::{map, map_res}};
use strum_macros::{EnumIter, FromRepr};
use serde_derive::{Serialize, Deserialize};
use ansiterm::Colour::*;
use chrono::TimeZone;

pub mod util;
use util::*;

pub trait UserInputtable {
    fn inputtable_string(&self) -> String;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub struct Description(pub String);
impl Display for Description {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", White.bold().paint(&self.0))
    }
}

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
}
impl UserInputtable for Color {
    fn inputtable_string(&self) -> String {
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
        }.to_owned()
    }
}
impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.paint(self.inputtable_string()))
    }
}
impl Eq for Color {}
impl Ord for Color {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.into_index().cmp(&other.0.into_index())
    }
}
impl PartialOrd for Color {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.color.0.paint(&self.name))
    }
}
impl Eq for Group {}
impl Ord for Group {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.name.cmp(&other.name)
    }
}
impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq)]
pub struct GroupRef(pub Rc<RefCell<Group>>);
impl UserInputtable for GroupRef {
    fn inputtable_string(&self) -> String {
        self.0.borrow().name.clone()
    }
}
impl Display for GroupRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.borrow().fmt(f)
    }
}
impl Eq for GroupRef {}
impl Ord for GroupRef {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.borrow().cmp(&other.0.borrow())
    }
}
impl PartialOrd for GroupRef {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Clone for GroupRef {
    fn clone(&self) -> Self {
        GroupRef(Rc::clone(&self.0))
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Instant(chrono::DateTime<chrono::Local>);
impl Instant {
    pub fn from_input(input: &str) -> io::Result<Self> {
        let ndt = chrono::NaiveDateTime::parse_from_str(input, "%F %-H:%M:%S").map_err(|_| invalid_input_error(&format!("invalid date & time: {}", input)))?;
        chrono::Local.from_local_datetime(&ndt).single().map(|dt| Instant(dt)).ok_or_else(|| invalid_input_error(&format!("this date & time is invalid because it falls on a daylight savings time border: {}", input)))
    }
}
impl Display for Instant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = self.0.format("%a %b %-d, %Y at %-I:%M:%S %P").to_string();
        write!(f, "{}", BrightYellow.paint(text))
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, EnumIter, FromRepr)]
pub enum Urgency {
    High,
    Medium,
    Low,
    LongTerm,
}
impl UserInputtable for Urgency {
    fn inputtable_string(&self) -> String {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
            Self::LongTerm => "long-term",
        }.to_owned()
    }
}
impl Display for Urgency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color = match self {
            Self::High => Red,
            Self::Medium => Yellow,
            Self::Low => Green,
            Self::LongTerm => Blue,
        };
        write!(f, "{}", color.paint(self.inputtable_string()))
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, EnumIter, FromRepr)]
pub enum Progress {
    InProgress,
    Done,
    Abandoned,
}
impl UserInputtable for Progress {
    fn inputtable_string(&self) -> String {
        match self {
            Self::InProgress => "in progress",
            Self::Done => "done",
            Self::Abandoned => "abandoned",
        }.to_owned()
    }
}
impl Display for Progress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color = match self {
            Self::InProgress => BrightBlue.normal(),
            Self::Done => BrightGreen.normal(),
            Self::Abandoned => Red.dimmed(),
        };
        write!(f, "{}", color.paint(self.inputtable_string()))
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct TodoItem {
    pub description: Description,
    pub group: Option<GroupRef>,
    pub due: Option<Instant>,
    pub urgency: Urgency,
    pub progress: Progress,
    pub created: Instant,
}
impl TodoItem {
    pub fn from(description: Description, group: Option<GroupRef>, due: Option<Instant>, urgency: Urgency) -> Self {
        Self { description, group, due, urgency, progress: Progress::InProgress, created: Instant(chrono::Local::now()) }
    }
}
impl Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let group = match &self.group {
            Some(g) => &format!("[{}] ", g.0.borrow()),
            None => "",
        };
        let due_date = match &self.due {
            Some(dt) => &format!(" due {}", dt),
            None => "",
        };
        write!(f, "{}{}{} ({} urgency) is {}", group, self.description, due_date, self.urgency, self.progress)
    }
}
impl UserInputtable for TodoItem {
    fn inputtable_string(&self) -> String {
        self.description.0.clone()
    }
}
impl Ord for TodoItem {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.progress.cmp(&other.progress).then(self.due.cmp(&other.due).then(self.urgency.cmp(&other.urgency)).then(self.description.cmp(&other.description)))
    }
}
impl PartialOrd for TodoItem {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct TodoData {
    pub groups: Vec<GroupRef>,
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
    desc: Description,
    group_index: Option<usize>,
    due: Option<Instant>,
    urgency: Urgency,
    progress: Progress,
    created: Instant,
}

#[derive(Serialize, Deserialize)]
/// necessary because `Rc`s don't serialize well :(
struct TodoDataSaved {
    groups: Vec<Group>,
    todos: Vec<TodoItemSaved>,
}
impl TodoDataSaved {
    fn from_unsaved(data: &TodoData) -> TodoDataSaved {
        let groups = data.groups.iter().map(|g| g.0.borrow().clone()).collect();
        let todos = data.todos.iter().map(|t| TodoItemSaved {
            desc: t.description.clone(),
            group_index: t.group.as_ref().and_then(|tg| data.groups.iter().position(|g| Rc::ptr_eq(&tg.0, &g.0))),
            due: t.due.clone(),
            urgency: t.urgency.clone(),
            progress: t.progress.clone(),
            created: t.created.clone(),
        }).collect();
        TodoDataSaved { groups, todos }
    }
    fn to_unsaved(self) -> TodoData {
        let groups: Vec<_> = self.groups.into_iter().map(|g| GroupRef(Rc::new(RefCell::new(g)))).collect();
        let todos = self.todos.into_iter().map(|t| TodoItem {
            description: t.desc,
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
    GroupRemove,
    TodoAdd,
    TodoEdit,
    TodoList,
    TodoRemove,
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
                                tag_no_case("remove"),
                                tag_no_case("rm"),
                            )
                        )
                    ),
                    |(entity, _, action)| match (entity, action) {
                        ("group" | "g", "add") => Ok(Self::GroupAdd),
                        ("group" | "g", "edit") => Ok(Self::GroupEdit),
                        ("group" | "g", "list" | "ls") => Ok(Self::GroupList),
                        ("group" | "g", "remove" | "rm") => Ok(Self::GroupRemove),
                        ("todo" | "t", "add") => Ok(Self::TodoAdd),
                        ("todo" | "t", "edit") => Ok(Self::TodoEdit),
                        ("todo" | "t", "list" | "ls") => Ok(Self::TodoList),
                        ("todo" | "t", "remove" | "rm") => Ok(Self::TodoRemove),
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