use crate::get_input::input;

/// Parses a string representing an integer.
/// Works similarly to int(x, 0) in Python.
///
/// # Examples
///
/// ```rust
/// use assemulator::parse_int;
/// assert_eq!(parse_int("0x10"), Ok(16));
/// assert_eq!(parse_int("0b1010"), Ok(10));
/// assert_eq!(parse_int("0o10"), Ok(8));
/// assert_eq!(parse_int("10"), Ok(10));
/// assert_eq!(parse_int("-1"), Ok(u64::MAX));
/// assert_eq!(parse_int("-0b101"), Ok(u64::MAX - 4));
/// ```
#[allow(clippy::option_if_let_else)]
pub fn parse_int(text: &str) -> Result<u64, String> {
    let value = if let Some(x) = text.strip_prefix('-') {
        Some(parse_int(x).map(|val| (!val) + 1)?)
    } else if let Some(x) = text.strip_prefix("0x") {
        u64::from_str_radix(x, 16).ok()
    } else if let Some(x) = text.strip_prefix("0b") {
        u64::from_str_radix(x, 2).ok()
    } else if let Some(x) = text.strip_prefix("0o") {
        u64::from_str_radix(x, 8).ok()
    } else {
        text.parse().ok()
    };

    value.ok_or_else(|| format!("Invalid integer: {text}"))
}

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
