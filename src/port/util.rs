use crate::{get_input::input, parse_int::parse_int};

/// Reads an int from user input.
pub fn read_int<T: TryFrom<u64>>() -> T {
    loop {
        let text: String = input("> ");
        match parse_int(text.trim())
            .and_then(|x| T::try_from(x).map_err(|_err| "Invalid input".to_owned()))
        {
            Ok(x) => break x,
            Err(err) => {
                println!("{err}");
            }
        }
    }
}
