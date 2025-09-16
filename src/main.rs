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
        let mut vec = vec![];
        let mut current_literal: Option<String> = None;
        for string in line.split(" ").map(str::to_lowercase) {
            let s = string.as_str();
            match current_literal {
                Some(cl) => {
                    match s.strip_suffix('"') {
                        Some(post_stripped) => {
                            vec.push(TodoToken::Literal(cl + " " + post_stripped));
                            current_literal = None;
                        },
                        None => current_literal = Some(cl + " " + s),
                    }
                },
                None => {
                    match s {
                        "add" | "create" => vec.push(TodoToken::Add),
                        "remove" | "delete" => vec.push(TodoToken::Remove),
                        "under" => vec.push(TodoToken::Category),
                        "due" => vec.push(TodoToken::Due),
                        "urgency" => vec.push(TodoToken::Urgency),
                        "progress" => vec.push(TodoToken::Progress),
                        s => {
                            match s.strip_prefix('"') {
                                Some(pre_stripped) => {
                                    match pre_stripped.strip_suffix('"') {
                                        Some(stripped) => {
                                            vec.push(TodoToken::Literal(String::from(stripped)));
                                            current_literal = None;
                                        },
                                        None => current_literal = Some(String::from(pre_stripped)),
                                    }
                                },
                                None => vec.push(TodoToken::Literal(string)),
                            }
                        },
                    }
                },
                
            }
        }
        vec
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
    
    fn from(line: &str) -> Result<TodoItem, ()> {
        let mut description;
        let mut category;
        let mut time_due = None;
        let mut urgency = None;
        // let mut time_created;
        let mut progress = Progress::InProgress;

        let mut tokens = TodoToken::vec_from(line).into_iter();
        while let Some(token) = tokens.next() {
            match token {
                TodoToken::Add => todo!(),
                TodoToken::Remove => todo!(),
                TodoToken::Edit => todo!(),
                TodoToken::Category => todo!(),
                TodoToken::Due => todo!(),
                TodoToken::Urgency => todo!(),
                TodoToken::Progress => todo!(),
                TodoToken::Literal(string) => todo!(),
            }
        }

        Ok(TodoItem {
            description,
            category,
            time_due,
            urgency,
            progress,
        })
    }
}

fn main() {
    let mut todos = vec![];
    loop {
        println!("add an item to your todo list! hahaha lol...");
        let line = match io::read_to_string(io::stdin()) {
            Ok(string) => string,
            Err(_) => continue,
        };
        todos.push(TodoItem::from(&line));
    }
}
