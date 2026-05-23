use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag_no_case, character::complete::multispace1, combinator::map
};

pub enum Command {
    CategoryAdd,
    CategoryEdit,
    TodoAdd,
    TodoEdit,
    Quit,
}

pub fn parse_command(input: &str) -> IResult<&str, Command> {
    // regex: "(q|quit|(category|todo)\s+(add|edit))"
    alt(
        (
            map(
                (
                    alt(
                        (
                            tag_no_case("category"),
                            tag_no_case("todo")
                        )
                    ),
                    multispace1,
                    alt(
                        (
                            tag_no_case("add"),
                            tag_no_case("edit")
                        )
                    )
                ),
                |(entity, _, action)| match (entity, action) {
                    ("category", "add") => Command::CategoryAdd,
                    ("category", "edit") => Command::CategoryEdit,
                    ("todo", "add") => Command::TodoAdd,
                    ("todo", "edit") => Command::TodoEdit,
                    _ => unreachable!()
                }
            ),
            map(
                alt(
                    (
                        tag_no_case("q"),
                        tag_no_case("quit")
                    )
                ),
                |_| Command::Quit
            )
        )
    ).parse(input)
}