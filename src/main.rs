use std::{io, time};
use ansiterm;

pub type Color = ansiterm::Colour;

pub enum CategoryToken {
    Add,
    Remove,
    Edit,
    Color,
    Literal(String),
}

pub struct Category {
    name: String,
    color: Color,
}

pub enum Urgency {
    Low,
    Medium,
    High,
    LongTerm,
}

pub enum Progress {
    InProgress,
    Done,
    FuckIt,
}

pub enum TodoToken {
    Add,
    Remove,
    Edit,
    Category,
    Due,
    Urgency,
    Progress,
    Literal(String),
}
impl TodoToken {
    fn vec_from(line: &str) -> Vec<TodoToken> {
        line.split(" ").map(str::to_lowercase).map(|s| {
            match s.as_str() {
                "add" | "create" => TodoToken::Add,
                "remove" | "delete" => TodoToken::Remove,
                "under" => TodoToken::Category,
                "due" => TodoToken::Due,
                "urgency" => TodoToken::Urgency,
                "progress" => TodoToken::Progress,
                s => TodoToken::Literal(String::from(s)),
            }
        }).collect()
    }
}

pub struct TodoItem {
    description: String,
    category: Option<Category>,
    time_due: Option<u64>,
    urgency: Option<Urgency>,

    progress: Progress,
}
impl TodoItem {
    fn new(description: String, category: Option<Category>, time_due: Option<u64>, urgency: Option<Urgency>) -> TodoItem {
        let now = time::SystemTime::now();
        // let since_epoch = now.duration_since(time::UNIX_EPOCH).expect("hello this is bad :)");
        TodoItem {
            description,
            category,
            time_due,
            urgency,

            // time_created: since_epoch.as_secs(),
            progress: Progress::InProgress,
        }
    }
    
    fn from_line(line: &str) -> io::Result<TodoItem> {
        let split = line.split(" ");

        let mut description;
        let mut category;
        let mut time_due = None;
        let mut urgency = None;
        let mut time_created;
        let mut progress = Progress::InProgress;

        while let Some(arg) = split.next() {
            match token {
                "add" => {
                    let token2 = match split.next() {
                        Some(s) => s,
                        None => eprintln!("bad!"),
                    };

                    desc
                },
            }
        }

        Err("hi");
    }
}

fn main() {
    loop {
        println!("add an item to your todo list! hahaha lol...");
        let args = match io::read_to_string(io::stdin()) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let mut split = args.trim();
        
    }
}
