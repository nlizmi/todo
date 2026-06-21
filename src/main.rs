#![allow(const_item_mutation)]

use strum::IntoEnumIterator;
use todo::*;
use std::{cell::RefCell, io, process::exit, rc::Rc};
use nom::Finish;

pub fn prompt_user(data: &mut TodoData) -> io::Result<()> {
    let mut buffer = String::new();
    util::read_input(&mut buffer)?;

    let (remaining, parsed) = Command::parse_from(&buffer).finish().map_err(|_| util::invalid_input_error(&format!("invalid command: {}\nFor a list of commands, type: <help/h>", buffer)))?;

    if !remaining.is_empty() {
        return Err(util::invalid_input_error(&format!("you typed some extra stuff: {}", remaining)));
    }

    match parsed {
        Command::GroupAdd => {
            println!("You're creating a new group!");

            let name = util::choose(&|s| Ok(s.to_owned()), "Name", &mut buffer)?;
            let color = *util::choose_from(&mut Color::variants, "Color", &mut buffer)?;

            let group = GroupRef(Rc::new(RefCell::new(Group::from(name, color))));
            println!("\nDone! Created group: {}", group.0.borrow());
            data.groups.push(group);
            data.save()?;
        },
        Command::GroupEdit => {
            println!("You're editing an existing group!");

            let group = util::choose_from(&mut data.groups, "Group to edit", &mut buffer)?.clone().0;

            let mut choices: Vec<_> = vec![
                ("name", format!("Name (currently {})", group.borrow().name)),
                ("color", format!("Color (currently {})", group.borrow().color)),
            ].into_iter().map(|(c, i)| util::ChoiceInfoPair(c.to_owned(), i)).collect();
            let choice = util::choose_from(&mut choices, "Property to change", &mut buffer)?;
            match choice.0.as_str() {
                "name" => group.borrow_mut().name = util::choose(&|s| Ok(s.to_owned()), "New name", &mut buffer)?,
                "color" => group.borrow_mut().color = *util::choose_from(&mut Color::variants, "New color", &mut buffer)?,
                _ => unreachable!(),
            }

            println!("\nDone! Edited group: {}", group.borrow());
            data.save()?;
        },
        Command::GroupList => {
            println!("{}", util::sorted_options_as_string(&mut data.groups));
        },
        Command::TodoAdd => {
            println!("You're creating a new todo item!");

            let desc = util::choose(&|s| Ok(Description(s.to_owned())), "Description", &mut buffer)?;
            let group = util::opt_choose_from(&mut data.groups, "Group", &mut buffer)?.cloned();
            let due = util::opt_choose(&Datum::from_input, "Due date and time (format YYYY-MM-DD HH:MM:SS)", &mut buffer)?;
            let urgency = util::choose_from(&mut Urgency::iter().collect::<Vec<_>>(), "Urgency", &mut buffer)?.clone();

            let todo = TodoItem::from(desc, group, due, urgency);
            println!("\nDone! Created todo item: {}", todo);
            data.todos.push(todo);
            data.save()?;
        },
        Command::TodoEdit => {
            println!("You're editing an existing todo item!");

            let todo = util::choose_from(&mut data.todos, "Todo item to edit", &mut buffer)?;

            let mut choices: Vec<_> = vec![
                ("desc", format!("Description (currently {})", todo.desc)),
                ("group", format!("Group (currently {})", match &todo.group {
                    Some(g) => g.0.borrow().to_string(),
                    None => "unassigned".to_owned(),
                })),
                ("due", format!("Due date (currently {})", match &todo.due {
                    Some(d) => d.to_string(),
                    None => "unassigned".to_owned(),
                })),
                ("urg", format!("Urgency (currently {})", todo.urgency)),
                ("prog", format!("Progress (currently {})", todo.progress))
            ].into_iter().map(|(c, i)| util::ChoiceInfoPair(c.to_owned(), i)).collect();
            let choice = util::choose_from(&mut choices, "Property to change", &mut buffer)?;
            match choice.0.as_str() {
                "desc" => todo.desc = util::choose(&|s| Ok(Description(s.to_owned())), "New description", &mut buffer)?,
                "group" => todo.group = util::opt_choose_from(&mut data.groups, "New group", &mut buffer)?.cloned(),
                "due" => todo.due = util::opt_choose(&Datum::from_input, "New due date and time (format YYYY-MM-DD HH:MM:SS)", &mut buffer)?,
                "urg" => todo.urgency = util::choose_from(&mut Urgency::iter().collect::<Vec<_>>(), "New urgency", &mut buffer)?.clone(),
                "prog" => todo.progress = util::choose_from(&mut Progress::iter().collect::<Vec<_>>(), "New progress", &mut buffer)?.clone(),
                _ => unreachable!(),
            }

            println!("\nDone! Edited todo item: {}", todo);
            data.save()?;
        },
        Command::TodoList => {
            print_todos(data);
        },
        Command::Help => print_help(),
        Command::Quit => exit(0),
    }

    Ok(())
}

fn print_help() {
    println!("Available commands:\n  - <todo/t> <add/edit/<list/ls>>\n  - <group/g> <add/edit/<list/ls>>\n  - <help/h>\n  - <quit/q/exit>");
}

fn print_todos(data: &mut TodoData) {
    println!("{}", util::sorted_options_as_string(&mut data.todos));
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
    print_todos(&mut data);
    loop {
        println!();
        match prompt_user(&mut data) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        }
    }
}
