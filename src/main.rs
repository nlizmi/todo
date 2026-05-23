use ansiterm::Color;
use chrono::{DateTime, Utc};
use todo::*;
use std::{io, process::exit, rc::Rc};
use nom::Finish;

fn read_input(buffer: &mut String) -> io::Result<()> {
    buffer.clear();
    io::stdin().read_line(buffer)?;
    *buffer = buffer.trim().to_owned();
    Ok(())
}

fn invalid_input_error(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, msg)
}

fn options_to_string(iter: impl Iterator<Item = (usize, String)>) -> String {
    iter.map(|(i, s)| format!("{}.\t{}", i, s)).fold(String::new(), |a, b| a + &b + "\n")
}

pub fn prompt_and_execute(data: &mut TodoData) -> io::Result<()> {
    let mut buffer = String::new();
    read_input(&mut buffer)?;

    let (remaining, parsed) = Command::parse_from(&buffer).finish().map_err(|e| invalid_input_error(&e.to_string()))?;

    if !remaining.is_empty() {
        return Err(invalid_input_error(&format!("you typed some extra stuff: {}", remaining)));
    }

    match parsed {
        Command::CategoryAdd => {
            println!("You're creating a new category!");

            println!("Name?");
            read_input(&mut buffer)?;
            let name = buffer.clone();

            println!("Color? Options are:\n{}", options_to_string(Color::options()));
            read_input(&mut buffer)?;
            let color = Color::from_input(&buffer).ok_or(invalid_input_error(&format!("invalid color: {}", buffer)))?;

            data.categories.push(Rc::new(Category::from(name, color)));
        },
        Command::CategoryEdit => todo!(),
        Command::CategoryList => todo!(),
        Command::TodoAdd => {
            println!("You're creating a new todo list item!");

            println!("Description?");
            read_input(&mut buffer)?;
            let desc = buffer.clone();

            println!("Category? Options are:\n{}", options_to_string(Category::options(data)));
            read_input(&mut buffer)?;
            let category = Category::from_data(data, &buffer).ok_or(invalid_input_error(&format!("invalid category: {}", buffer)))?;

            println!("Due date and/or time?");
            read_input(&mut buffer)?;
            let due = if buffer.is_empty() {
                None
            } else {
                Some(DateTime::<Utc>::from_input(&buffer).ok_or(invalid_input_error(&format!("invalid date/time: {}", buffer)))?)
            };

            println!("Urgency? Options are:\n{}", options_to_string(Urgency::options()));
            read_input(&mut buffer)?;
            let urgency = Urgency::from(&buffer).ok_or(invalid_input_error(&format!("invalid urgency: {}", buffer)))?;

            data.todos.push(Todo::from(desc, category, due, urgency));
        },
        Command::TodoEdit => todo!(),
        Command::TodoList => todo!(),
        Command::Quit => exit(0),
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let mut data = todo::TodoData::new();
    loop {
        println!("todo list cli");
        match prompt_and_execute(&mut data) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        }
    }
}
