use std::{fmt::Display, io, path};

#[derive(PartialEq, Eq)]
pub struct ChoiceInfoPair<C, I>(pub C, pub I);
impl <C: Display, I: Display> Display for ChoiceInfoPair<C, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}
impl <C: Ord, I: Eq> Ord for ChoiceInfoPair<C, I> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
impl <C: Ord, I: Eq> PartialOrd for ChoiceInfoPair<C, I> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

pub fn choose<T>(transform: impl Fn(&str) -> io::Result<T>, prompt: &str, input: &mut String) -> io::Result<T> {
    println!("{}?", prompt);
    read_input(input)?;
    transform(input)
}

pub fn opt_choose<T>(transform: impl Fn(&str) -> io::Result<T>, prompt: &str, input: &mut String) -> io::Result<Option<T>> {
    println!("{}?", prompt);
    read_input(input)?;
    if input.is_empty() {
        Ok(None)
    } else {
        transform(input).map(|x| Some(x))
    }
}

pub fn choose_from<'a, T: Ord + Display>(options: &'a mut [T], stringify: impl Fn(&T) -> String, prompt: &str, input: &mut String) -> io::Result<&'a T> {
    println!("\n{}? Options are:\n{}", prompt, sorted_options_as_string(options));
    read_input(input)?;
    get_choice_index(options, stringify, input).map(|i| &options[i])
}

pub fn opt_choose_from<'a, T: Ord + Display>(options: &'a mut [T], stringify: impl Fn(&T) -> String, prompt: &str, input: &mut String) -> io::Result<Option<&'a T>> {
    println!("\n{} (optional)? Options are:\n{}", prompt, sorted_options_as_string(options));
    read_input(input)?;
    if input.is_empty() {
        Ok(None)
    } else {
        get_choice_index(options, stringify, input).map(|i| Some(&options[i]))
    }
}

pub fn get_choice_index<'a, T>(options: &'a [T], stringify: impl Fn(&T) -> String, input: &str) -> io::Result<usize> {
    match input.parse::<usize>() {
        Ok(i) if i < options.len() => Ok(i),
        Ok(i) => Err(invalid_input_error(&format!("index out of bounds: {}", i))),
        Err(_) => options.iter().position(|option| input.to_lowercase() == stringify(option).to_lowercase()).ok_or_else(|| invalid_input_error(&format!("invalid choice: {}", input)))
    }
}

pub fn sorted_options_as_string<T: Ord + Display>(options: &mut [T]) -> String {
    if !options.is_sorted() { options.sort(); }
    let mut iter = options.iter().peekable();
    if iter.peek().is_none() { return "none :(".to_owned(); }
    let mut s = iter.enumerate().map(|(i, s)| format!("{}.\t{}", i, s)).fold(String::new(), |a, b| a + &b + "\n");
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
