use chrono::NaiveDateTime;
use todo::*;
use std::io;
use nom::Finish;

fn read_input(buffer: &mut String) {
    io::stdin().read_line(buffer)?;
    buffer.trim();
}

pub fn execute_input(input: &str, data: &mut TodoData) -> io::Result<()> {
    let (remaining, parsed) = Command::parse_from(input).finish().map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidInput, e.to_string())
    })?;

    if !remaining.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("you typed some extra stuff: {}", remaining)));
    }

    match parsed {
        Command::CategoryAdd => {
            println!("You're creating a new category!");

            let mut buffer = String::new();

            println!("Name?");
            read_input(&mut buffer);
            let name = buffer.clone();

            println!("Color? Options are: {}", );
            read_input(&mut buffer);
            let color = Color::from(&buffer);

            data.categories.push(Category::from(name, color));
        },
        Command::CategoryEdit => todo!(),
        Command::TodoAdd => {
            println!("You're creating a new todo list item!");

            println!("Description?");
            let mut buffer = String::new();
            read_input(&mut buffer);
            let desc = buffer.clone();

            println!("Category? Options are: {:?}", Category::options(data));
            read_input(&mut buffer);
            let category = Category::from_data(data, &buffer)?;

            println!("Due date and/or time?");
            read_input(&mut buffer);
            let due = DateTime::from(&buffer)?;

            println!("Urgency? Options are: {:?}", Urgency::options());
            read_input(&mut buffer);
            let urgency = Urgency::from(&buffer)?;

            data.todos.push(Todo::from(desc, category, due, urgency));
        },
        Command::TodoEdit => todo!(),
        Command::Quit => todo!()
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let mut data = todo::TodoData::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        execute_input(&input, &mut data)?;
    }
}
