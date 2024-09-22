/// Parses a string representing an integer.
/// Works similarly to int(x, 0) in Python.
///
/// Doesn't support negative values.
///
/// # Examples
///
/// ```rust
/// use assemulator::parse_int;
/// assert_eq!(parse_int("0x10"), Ok(16));
/// assert_eq!(parse_int("0b1010"), Ok(10));
/// assert_eq!(parse_int("0o10"), Ok(8));
/// assert_eq!(parse_int("10"), Ok(10));
/// ```
#[allow(clippy::option_if_let_else)]
fn parse_int(text: &str) -> Result<u64, String> {
    let value = if let Some(x) = text.strip_prefix("0x") {
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

// these should be sorted by length for maximal munch
const OPERATORS: [(&str, usize); 18] = [
    ("||", 0),
    ("&&", 1),
    ("==", 2),
    ("!=", 2),
    ("<=", 2),
    (">=", 2),
    ("<<", 9),
    (">>", 9),
    ("<", 2),
    (">", 2),
    ("|", 3),
    ("^", 4),
    ("&", 5),
    ("+", 10),
    ("-", 10),
    ("*", 20),
    ("/", 20),
    ("%", 20),
];

/// Assumes input is a correctly formed math expression.
pub fn parse(expression: &str) -> Result<u64, String> {
    let mut parser = Parser::new(expression);
    parser.parse_expression(0)
}

struct Parser<'a> {
    expression: &'a str,
    i: usize,
}

impl<'a> Parser<'a> {
    const fn new(expression: &'a str) -> Self {
        Parser { expression, i: 0 }
    }

    /// Returns true if the next token is `tok`.
    fn accept(&mut self, tok: &str) -> bool {
        if self.expression[self.i..].starts_with(tok) {
            self.i += tok.len();
            true
        } else {
            false
        }
    }

    fn accept_number(&mut self) -> Option<u64> {
        if self.expression[self.i..].starts_with(char::is_numeric) {
            let start = self.i;
            while self.expression[self.i..].starts_with(char::is_alphanumeric)
                || self.expression[self.i..].starts_with('_')
            {
                self.i += 1;
            }
            let num = &self.expression[start..self.i];
            Some(parse_int(num).unwrap())
        } else {
            None
        }
    }

    fn expect(&mut self, tok: &str) -> Result<(), String> {
        if self.accept(tok) {
            Ok(())
        } else {
            Err(format!("Expected {} {}", tok, &self.expression[self.i..]))
        }
    }

    fn find_operator(&self) -> Option<(&'a str, usize)> {
        OPERATORS
            .iter()
            .find_map(|(x, y)| self.expression[self.i..].starts_with(x).then_some((*x, *y)))
    }

    pub fn parse_expression(&mut self, precedence: usize) -> Result<u64, String> {
        let mut left = self.parse_factor()?;

        while let Some((op, prec)) = self.find_operator() {
            if prec < precedence {
                break;
            }
            self.accept(op);
            let right = self.parse_expression(prec + 1)?;

            left = match op {
                "+" => left.wrapping_add(right),
                "-" => left.wrapping_sub(right),
                "*" => left.wrapping_mul(right),
                "/" => left.wrapping_div(right),
                "%" => left.wrapping_rem(right),
                "&" => left & right,
                "|" => left | right,
                "^" => left ^ right,
                "<<" => left.wrapping_shl(right as u32),
                ">>" => left.wrapping_shr(right as u32),
                "<" => u64::from(left < right),
                ">" => u64::from(left > right),
                "<=" => u64::from(left <= right),
                ">=" => u64::from(left >= right),
                "==" => u64::from(left == right),
                "!=" => u64::from(left != right),
                "&&" => u64::from(left != 0 && right != 0),
                "||" => u64::from(left != 0 || right != 0),
                _ => panic!(),
            };
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<u64, String> {
        if self.accept("(") {
            let result = self.parse_expression(0)?;
            self.expect(")")?;
            Ok(result)
        } else if self.accept("-") {
            self.parse_factor().map(|x| (!x).wrapping_add(1)) // negate by 2s complement
        } else if self.accept("~") {
            self.parse_factor().map(|x| !x)
        } else if self.accept("!") {
            self.parse_factor().map(|x| u64::from(x == 0))
        } else if let Some(number) = self.accept_number() {
            Ok(number)
        } else if self.i >= self.expression.len() {
            Err("Unexpected end of input".to_string())
        } else {
            Err(format!(
                "Expected number, got: {}",
                &self.expression[self.i..]
            ))
        }
    }
}
