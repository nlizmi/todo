use strum::IntoEnumIterator;
use todo::*;
use std::{io, process::exit, rc::Rc};
use nom::Finish;

pub fn prompt_user(data: &mut TodoData) -> io::Result<()> {
    let mut buffer = String::new();
    util::read_input(&mut buffer)?;

    let (remaining, parsed) = Command::parse_from(&buffer).finish().map_err(|_| util::invalid_input_error(&format!("invalid command: {}", buffer)))?;

    if !remaining.is_empty() {
        return Err(util::invalid_input_error(&format!("you typed some extra stuff: {}", remaining)));
    }

    match parsed {
        Command::GroupAdd => {
            println!("You're creating a new group!");

            println!("\nName?");
            util::read_input(&mut buffer)?;
            let name = buffer.clone();

            println!("\nColor? Options are:\n{}", util::numbered_list_as_string(Color::variants().iter()));
            util::read_input(&mut buffer)?;
            let color = Color::from_input(&buffer).ok_or_else(|| util::invalid_input_error(&format!("invalid color: {}", buffer)))?;

            let group = Rc::new(Group::from(name, color));
            println!("\nDone! Created group: {}", group);
            data.groups.insert(group);
            data.save()?;
        },
        Command::GroupEdit => todo!(),
        Command::GroupList => {
            if data.groups.is_empty() { println!("No groups present! create one by typing: <group/g> add"); }
            for (i, group) in data.groups.iter().enumerate() {
                println!("{}.\t{}", i, group);
            }
        },
        Command::TodoAdd => {
            println!("You're creating a new todo list item!");

            println!("\nDescription?");
            util::read_input(&mut buffer)?;
            let desc = buffer.clone();

            println!("\nGroup (optional)? Options are:\n{}", util::numbered_list_as_string(data.groups.iter()));
            util::read_input(&mut buffer)?;
            let group = if buffer.is_empty() {
                None
            } else {
                Some(Group::from_data(data, &buffer).ok_or_else(|| util::invalid_input_error(&format!("invalid group: {}", buffer)))?)
            };

            println!("\nDue date and/or time (optional)?");
            util::read_input(&mut buffer)?;
            let due = if buffer.is_empty() {
                None
            } else {
                Some(util::datetime_from_input(&buffer).ok_or_else(|| util::invalid_input_error(&format!("invalid date/time: {}", buffer)))?)
            };

            println!("\nUrgency? Options are:\n{}", util::numbered_list_as_string(Urgency::iter()));
            util::read_input(&mut buffer)?;
            let urgency = Urgency::from(&buffer).ok_or_else(|| util::invalid_input_error(&format!("invalid urgency: {}", buffer)))?;

            let todo = Todo::from(desc, group, due, urgency);
            println!("Done! Created todo item: {}", todo);
            data.todos.insert(todo);
            data.save()?;
        },
        Command::TodoEdit => todo!(),
        Command::TodoList => {
            if data.todos.is_empty() { println!("No todo items present! create one by typing: <todo/t> add"); }
            println!("{}", util::numbered_list_as_string(data.todos.iter()));
        },
        Command::Help => print_help(),
        Command::Quit => exit(0),
    }

    Ok(())
}

fn print_help() {
    println!("Available commands:\n  - <todo/t> <add/edit/<list/ls>>\n  - <group/g> <add/edit/<list/ls>>\n  - <help/h>\n  - <quit/q/exit>");
}

fn main() {
    let mut data = TodoData::load();
    println!(r#"
 ,--.--------.   _,.---._                   _,.---._     
/==/,  -   , -\,-.' , -  `.   _,..---._   ,-.' , -  `.   
\==\.-.  - ,-./==/_,  ,  - \/==/,   -  \ /==/_,  ,  - \  
 `--`\==\- \ |==|   .=.     |==|   _   _\==|   .=.     | 
      \==\_ \|==|_ : ;=:  - |==|  .=.   |==|_ : ;=:  - | 
      |==|- ||==| , '='     |==|,|   | -|==| , '='     | 
      |==|, | \==\ -    ,_ /|==|  '='   /\==\ -    ,_ /  
      /==/ -/  '.='. -   .' |==|-,   _`/  '.='. -   .'   
      `--`--`    `--`--''   `-.`.____.'     `--`--''     
"#);
    print_help();
    loop {
        println!();
        match prompt_user(&mut data) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        }
    }
}
