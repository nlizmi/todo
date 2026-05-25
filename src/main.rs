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

            println!("\nName?");
            util::read_input(&mut buffer)?;
            let name = buffer.clone();

            let colors = Color::variants;
            println!("\nColor? Options are:\n{}", util::numbered_list_as_string(Color::variants.iter()));
            util::read_input(&mut buffer)?;
            let color = colors[util::get_choice_index(&colors, |c| c.as_str().to_owned(), &buffer)?];
            let color = util::choose_from(&Color::variants, |c| c.as_str().to_owned(), )

            let group = Rc::new(RefCell::new(Group::from(name, color)));
            println!("\nDone! Created group: {}", group.borrow());
            data.groups.push(group);
            data.save()?;
        },
        Command::GroupEdit => {
            println!("You're editing an existing group!");

            let groups = &mut data.groups;
            util::checked_sort(groups);
            println!("\nGroup to edit? Options are:\n{}", util::numbered_list_as_string(groups.iter().map(|g| g.borrow())));
            util::read_input(&mut buffer)?;
            let group = groups[util::get_choice_index(&groups, |g| g.borrow().name.clone(), &buffer)?].clone();

            let choices = vec![
                ("name", format!("Name (currently {})", group.borrow().name)),
                ("color", format!("Color (currently {})", group.borrow().color)),
            ];
            println!("\nWhat would you like to change? Options are:\n{}", util::numbered_list_as_string(choices.iter().map(|o| format!("{}: {}", o.0, o.1))));
            util::read_input(&mut buffer)?;
            let choice = &choices[util::get_choice_index(&choices, |o| o.0.to_owned(), &buffer)?];
            match choice.0 {
                "name" => {
                    println!("\nNew name?");
                    util::read_input(&mut buffer)?;
                    let name = buffer.clone();

                    group.borrow_mut().name = name;
                },
                "color" => {
                    let colors = Color::variants;
                    println!("\nNew color? Options are:\n{}", util::numbered_list_as_string(Color::variants.iter()));
                    util::read_input(&mut buffer)?;
                    let color = colors[util::get_choice_index(&colors, |c| c.as_str().to_owned(), &buffer)?];

                    group.borrow_mut().color = color;
                },
                _ => unreachable!(),
            }

            println!("\nDone! Edited group: {}", group.borrow());
            data.save()?;
        },
        Command::GroupList => {
            let groups: Vec<_> = data.groups.iter().map(|g| g.borrow()).collect();
            println!("{}", util::sorted_options_as_string(&mut groups));
        },
        Command::TodoAdd => {
            println!("You're creating a new todo item!");

            let desc = util::choose(|s| Ok(Description(s.to_owned())), "Description", &mut buffer)?;

            let group = util::opt_choose_from(&mut data.groups, |g| g.borrow().to_string(), "Group", &mut buffer)?.cloned();

            let due = util::opt_choose(Datum::from_input, "Due date and time (format YYYY-MM-DD HH:MM:SS)", &mut buffer)?;

            let urgency = util::choose_from(&mut Urgency::iter().collect::<Vec<_>>(), |u| u.as_str().to_owned(), "Urgency", &mut buffer)?.clone();

            let todo = TodoItem::from(desc, group, due, urgency);
            println!("\nDone! Created todo item: {}", todo);
            data.todos.push(todo);
            data.save()?;
        },
        Command::TodoEdit => {
            println!("You're editing an existing todo item!");

            let todo = util::choose_from(&mut data.todos, |t| t.desc.0.to_owned(), "Todo item to edit", &mut buffer)?;

            let choices: Vec<_> = vec![
                ("desc", format!("Description (currently {})", todo.desc)),
                ("group", format!("Group (currently {})", match todo.group.as_ref() {
                    Some(g) => g.borrow().to_string(),
                    None => "unassigned".to_owned(),
                })),
                ("due", format!("Due date (currently {})", match &todo.due {
                    Some(d) => d.to_string(),
                    None => "unassigned".to_owned(),
                })),
                ("urg", format!("Urgency (currently {})", todo.urgency)),
                ("prog", format!("Progress (currently {})", todo.progress))
            ].into_iter().map(|(c, i)| util::ChoiceInfoPair(c.to_owned(), i)).collect();
            let choice = util::choose_from(&mut choices, |c| c.0, "Property to change", &mut buffer)?;
            match choice.0.as_str() {
                "desc" => {
                    println!("\nNew description?");
                    util::read_input(&mut buffer)?;
                    let desc = Description(buffer.clone());
                    todo.desc = desc;
                },
                "group" => {
                    let group = util::opt_choose_from(&mut data.groups, |g| g.borrow().to_string(), "New group", &mut buffer)?.cloned();
                    todo.group = group;
                },
                "due" => {
                    println!("\nNew due date and time (optional)? Format is: YYYY-MM-DD HH:MM:SS\nYou can omit leading zeros for the hours.");
                    util::read_input(&mut buffer)?;
                    let due = if buffer.is_empty() {
                        None
                    } else {
                        Some(Datum::from_input(&buffer)?)
                    };

                    todo.due = due;
                },
                "urg" => {
                    let urgencies: Vec<_> = Urgency::iter().collect();
                    println!("\nNew urgency? Options are:\n{}", util::numbered_list_as_string(urgencies.iter()));
                    util::read_input(&mut buffer)?;
                    let urgency = urgencies[util::get_choice_index(&urgencies, |u| u.as_str().to_owned(), &buffer)?].clone();

                    todo.urgency = urgency;
                },
                "prog" => {
                    let progresses: Vec<_> = Progress::iter().collect();
                    println!("\nNew urgency? Options are:\n{}", util::numbered_list_as_string(progresses.iter()));
                    util::read_input(&mut buffer)?;
                    let progress = progresses[util::get_choice_index(&progresses, |p| p.as_str().to_owned(), &buffer)?].clone();

                    todo.progress = progress;
                },
                _ => unreachable!(),
            }

            println!("\nDone! Edited todo item: {}", todo);
            data.save()?;
        },
        Command::TodoList => {
            if data.todos.is_empty() {
                println!("No todo items present! create one by typing: <todo/t> add");
            } else {
                print_todos(data);
            }
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
    let todos = &mut data.todos;
    util::checked_sort(todos);
    println!("{}", util::numbered_list_as_string(todos.iter()));
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
