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
        Command::GroupAdd => {
            println!("You're creating a new group!\n");

            println!("\nName?");
            read_input(&mut buffer)?;
            let name = buffer.clone();

            println!("\nColor? Options are:\n{}", options_to_string(color_options()));
            read_input(&mut buffer)?;
            let color = color_from_input(&buffer).ok_or(invalid_input_error(&format!("invalid color: {}", buffer)))?;

            let group = Rc::new(Group::from(name, color));
            println!("\nDone! Created group: {}", group);
            data.groups.push(group);
        },
        Command::GroupEdit => todo!(),
        Command::GroupList => {
            if data.groups.is_empty() { println!("[no groups present! create one by typing: group add]"); }
            let mut groups = data.groups.clone();
            groups.sort_by(|a, b| a.name.cmp(&b.name));
            for (i, category) in groups.iter().enumerate() {
                println!("{}.\t{}", i, category);
            }
        },
        Command::TodoAdd => {
            println!("You're creating a new todo list item!");

            println!("\nDescription?");
            read_input(&mut buffer)?;
            let desc = buffer.clone();

            println!("\nGroup (optional)? Options are:\n{}", options_to_string(Group::options(data)));
            read_input(&mut buffer)?;
            let category = if buffer.is_empty() {
                None
            } else {
                Some(Group::from_data(data, &buffer).ok_or(invalid_input_error(&format!("invalid category: {}", buffer)))?)
            };

            println!("\nDue date and/or time (optional)?");
            read_input(&mut buffer)?;
            let due = if buffer.is_empty() {
                None
            } else {
                Some(datetime_from_input(&buffer).ok_or(invalid_input_error(&format!("invalid date/time: {}", buffer)))?)
            };

            println!("\nUrgency? Options are:\n{}", options_to_string(Urgency::options()));
            read_input(&mut buffer)?;
            let urgency = Urgency::from(&buffer).ok_or(invalid_input_error(&format!("invalid urgency: {}", buffer)))?;

            let todo = Todo::from(desc, category, due, urgency);
            println!("Done! Created todo item: {}", todo);
            data.todos.push(todo);
        },
        Command::TodoEdit => todo!(),
        Command::TodoList => {
            if data.todos.is_empty() { println!("[no todo items present! create one by typing: todo add]"); }
            let mut todos = data.todos.clone();
            todos.sort_by(|a, b| a.due.cmp(&b.due).then(a.urgency.cmp(&b.urgency)).then(a.desc.cmp(&b.desc)));
            for (i, todo) in todos.iter().enumerate() {
                println!("{}.\t{}", i, todo);
            }
        },
        Command::Help => print_help(),
        Command::Quit => exit(0),
    }

    Ok(())
}

fn print_help() {
    println!("Available commands:\n  - todo <add/edit/list>\n  - group <add/edit/list>\n  - <help/h>\n  - <quit/q>");
}

fn main() -> io::Result<()> {
    let mut data = todo::TodoData::new();
    println!("*** TODO LIST PROGRAM ***");
    print_help();
    loop {
        println!();
        match prompt_and_execute(&mut data) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        }
    }
}
