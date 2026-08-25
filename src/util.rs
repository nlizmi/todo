use std::{cmp, fmt::{self, Display}, io, path};

use crate::{UserInputtable};

#[derive(PartialEq, Eq)]
pub struct ChoiceInfoPair<I>(pub String, pub I);
impl <I: Display> Display for ChoiceInfoPair<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.0, self.1)
    }
}
impl <I> UserInputtable for ChoiceInfoPair<I> {
    fn inputtable_string(&self) -> String {
        self.0.clone()
    }
}
impl <I: Eq> Ord for ChoiceInfoPair<I> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
impl <I: Eq> PartialOrd for ChoiceInfoPair<I> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

pub fn choose<T, F>(transform: &F, prompt: &str, input: &mut String) -> io::Result<T>
where
    F: Fn(&str) -> io::Result<T>
{
    println!("\n{}?", prompt);
    read_input(input)?;
    if input.is_empty() {
        Err(invalid_input_error("choice can't be empty"))
    } else {
        transform(input)
    }
}

pub fn opt_choose<T, F>(transform: &F, prompt: &str, input: &mut String) -> io::Result<Option<T>>
where
    F: Fn(&str) -> io::Result<T>
{
    println!("\n{} (optional)?", prompt);
    read_input(input)?;
    if input.is_empty() {
        Ok(None)
    } else {
        transform(input).map(|x| Some(x))
    }
}

pub fn choose_index_from<T>(options: &mut [T], prompt: &str, input: &mut String) -> io::Result<usize>
where
    T: Ord + Display + UserInputtable
{
    println!("\n{}? Options are:\n{}", prompt, sorted_options_as_string(options));
    read_input(input)?;
    get_choice_index(options, input)
}

pub fn choose_from<'a, T>(options: &'a mut [T], prompt: &str, input: &mut String) -> io::Result<&'a mut T>
where
    T: Ord + Display + UserInputtable
{
    choose_index_from(options, prompt, input).map(|i| &mut options[i])
}


pub fn opt_choose_index_from<T>(options: &mut [T], prompt: &str, input: &mut String) -> io::Result<Option<usize>>
where
    T: Ord + Display + UserInputtable
{
    println!("\n{} (optional)? Options are:\n{}", prompt, sorted_options_as_string(options));
    read_input(input)?;
    if input.is_empty() {
        Ok(None)
    } else {
        get_choice_index(options, input).map(|i| Some(i))
    }
}

pub fn opt_choose_from<'a, T>(options: &'a mut [T], prompt: &str, input: &mut String) -> io::Result<Option<&'a mut T>>
where
    T: Ord + Display + UserInputtable
{
    opt_choose_index_from(options, prompt, input).map(|opt| opt.map(|i| &mut options[i]))
}

pub fn get_choice_index<T>(options: &mut [T], input: &str) -> io::Result<usize>
where
    T: UserInputtable
{
    match input.parse::<usize>() {
        Ok(i) if i > 0 && i <= options.len() => Ok(i - 1),
        Ok(i) => Err(invalid_input_error(&format!("index out of bounds: {}", i))),
        Err(_) => options.iter().position(|option| input.to_lowercase() == option.inputtable_string().to_lowercase()).ok_or_else(|| invalid_input_error(&format!("invalid choice: {}", input)))
    }
}

pub fn sorted_options_as_string<T>(options: &mut [T]) -> String
where 
    T: Ord + Display
{
    if !options.is_sorted() { options.sort(); }
    let mut iter = options.iter().peekable();
    if iter.peek().is_none() { return "none :(".to_owned(); }
    let mut s = iter.enumerate().map(|(i, x)| format!("{}.\t{}", i + 1, x)).fold(String::new(), |a, b| a + &b + "\n");
    s.pop();
    s
}

pub fn read_input(buffer: &mut String) -> io::Result<()> {
    buffer.clear();
    io::stdin().read_line(buffer)?;
    *buffer = buffer.trim().to_owned();
    Ok(())
}

pub fn invalid_input_error(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, msg)
}

pub fn data_path() -> io::Result<path::PathBuf> {
    dirs::data_dir().map(|dir| dir.join("nlizmi-todo").join("data.cbor")).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidFilename, "couldn't find the data directory"))
}
