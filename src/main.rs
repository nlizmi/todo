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
            print_groups(data);
            data.save()?;
        },
        Command::GroupEdit => {
            println!("You're editing an existing group!");

            let group = util::choose_from(&mut data.groups, "Group to edit", &mut buffer)?.0.clone();

            let mut choices: Vec<_> = vec![
                ("name", group.borrow().name.to_string()),
                ("color", group.borrow().color.to_string()),
            ].into_iter().map(|(c, i)| util::ChoiceInfoPair(c.to_owned(), i)).collect();
            let choice = util::choose_from(&mut choices, "Property to change", &mut buffer)?;
            match choice.0.as_str() {
                "name" => group.borrow_mut().name = util::choose(&|s| Ok(s.to_owned()), "New name", &mut buffer)?,
                "color" => group.borrow_mut().color = *util::choose_from(&mut Color::variants, "New color", &mut buffer)?,
                _ => unreachable!(),
            }

            println!("\nDone! Edited group: {}", group.borrow());
            print_groups(data);
            data.save()?;
        },
        Command::GroupList => {
            print_groups(data);
        },
        Command::GroupRemove => {
            let i = util::choose_index_from(&mut data.groups, "Group to remove", &mut buffer)?;
            let group = data.groups[i].0.clone();
            for todo in &mut data.todos {
                if let Some(g) = &todo.group && Rc::ptr_eq(&group, &g.0) {
                    todo.group = None;
                }
            }
            data.groups.remove(i);

            println!("\nDone! Removed group: {}", group.borrow());
            print_groups(data);
            data.save()?;
        },
        Command::TodoAdd => {
            println!("You're creating a new todo item!");

            let desc = util::choose(&|s| Ok(Description(s.to_owned())), "Description", &mut buffer)?;
            let group = util::opt_choose_from(&mut data.groups, "Group", &mut buffer)?.cloned();
            let due = util::opt_choose(&Instant::from_input, "Due date and time (format YYYY-MM-DD HH:MM)", &mut buffer)?;
            let urgency = util::choose_from(&mut Urgency::iter().collect::<Vec<_>>(), "Urgency", &mut buffer)?.clone();

            let todo = TodoItem::from(desc, group, due, urgency);
            println!("\nDone! Created todo item: {}", todo);
            data.todos.push(todo);
            print_todos(data);
            data.save()?;
        },
        Command::TodoEdit => {
            println!("You're editing an existing todo item!");

            let todo = util::choose_from(&mut data.todos, "Todo item to edit", &mut buffer)?;

            let mut choices: Vec<_> = vec![
                ("description", todo.description.to_string()),
                ("due", match &todo.due {
                    Some(d) => d.to_string(),
                    None => "unassigned".to_owned(),
                }),
                ("group", match &todo.group {
                    Some(g) => g.0.borrow().to_string(),
                    None => "unassigned".to_owned(),
                }),
                ("progress", todo.progress.to_string()),
                ("urgency", todo.urgency.to_string()),
            ].into_iter().map(|(c, i)| util::ChoiceInfoPair(c.to_owned(), i)).collect();
            let choice = util::choose_from(&mut choices, "Property to change", &mut buffer)?;
            match choice.0.as_str() {
                "description" => todo.description = util::choose(&|s| Ok(Description(s.to_owned())), "New description", &mut buffer)?,
                "due" => todo.due = util::opt_choose(&Instant::from_input, "New due date and time (format YYYY-MM-DD HH:MM)", &mut buffer)?,
                "group" => todo.group = util::opt_choose_from(&mut data.groups, "New group", &mut buffer)?.cloned(),
                "progress" => todo.progress = util::choose_from(&mut Progress::iter().collect::<Vec<_>>(), "New progress", &mut buffer)?.clone(),
                "urgency" => todo.urgency = util::choose_from(&mut Urgency::iter().collect::<Vec<_>>(), "New urgency", &mut buffer)?.clone(),
                _ => unreachable!(),
            }

            println!("\nDone! Edited todo item: {}", todo);
            print_todos(data);
            data.save()?;
        },
        Command::TodoList => {
            print_todos(data);
        },
        Command::TodoRemove => {
            let i = util::choose_index_from(&mut data.todos, "Todo item to remove", &mut buffer)?;
            let todo = data.todos[i].clone();
            data.todos.remove(i);

            println!("\nDone! Removed todo item: {}", todo);
            print_todos(data);
            data.save()?;
        }
        Command::Help => print_help(),
        Command::Quit => exit(0),
    }

    Ok(())
}

fn print_help() {
    println!("Available commands:\n  - <todo/t> <add / <edit/ed> / <list/ls> / <remove/rm>>\n  - <group/g> <add / <edit/ed> / <list/ls> / <remove/rm>>\n  - <help/h>\n  - <quit/q/exit>");
}

fn print_groups(data: &mut TodoData) {
    println!("Current groups:\n{}", util::sorted_options_as_string(&mut data.groups));
}

fn print_todos(data: &mut TodoData) {
    let mut current_todos = data.current_todos();
    println!("Current todos:\n{}", util::sorted_options_as_string(&mut current_todos));
    let diff = data.todos.len() - current_todos.len();
    if diff > 0 {
        let text = format!("(...and {} other finished tasks)", diff);
        println!("{}", ansiterm::Colour::DarkGray.paint(text));
    }
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
