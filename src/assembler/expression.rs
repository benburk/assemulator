use crate::parse_int::parse_int;

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

/// Evaluates a math expression. Whitespace must already be stripped.
pub fn parse(expression: &str) -> Result<u64, String> {
    let mut parser = Parser::new(expression);
    let value = parser.parse_expression(0)?;
    if parser.i < expression.len() {
        return Err(format!("Unexpected input: {}", &expression[parser.i..]));
    }
    Ok(value)
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

    fn accept_number(&mut self) -> Option<Result<u64, String>> {
        if self.expression[self.i..].starts_with(|c: char| c.is_ascii_digit()) {
            let start = self.i;
            while self.expression[self.i..].starts_with(|c: char| c.is_ascii_alphanumeric())
                || self.expression[self.i..].starts_with('_')
            {
                self.i += 1;
            }
            Some(parse_int(&self.expression[start..self.i]))
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
                "/" => left.checked_div(right).ok_or("Division by zero")?,
                "%" => left.checked_rem(right).ok_or("Division by zero")?,
                "&" => left & right,
                "|" => left | right,
                "^" => left ^ right,
                "<<" => left.wrapping_shl(u32::try_from(right % 64).unwrap_or_default()),
                ">>" => left.wrapping_shr(u32::try_from(right % 64).unwrap_or_default()),
                "<" => u64::from(left < right),
                ">" => u64::from(left > right),
                "<=" => u64::from(left <= right),
                ">=" => u64::from(left >= right),
                "==" => u64::from(left == right),
                "!=" => u64::from(left != right),
                "&&" => u64::from(left != 0 && right != 0),
                "||" => u64::from(left != 0 || right != 0),
                _ => unreachable!("operator missing from OPERATORS: {op}"),
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
            number
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

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn respects_precedence() {
        assert_eq!(parse("1+2*3"), Ok(7));
        assert_eq!(parse("(1+2)*3"), Ok(9));
        assert_eq!(parse("1<<2+1"), Ok(8));
        assert_eq!(parse("10-4-3"), Ok(3));
    }

    #[test]
    fn evaluates_comparisons() {
        assert_eq!(parse("2<=2"), Ok(1));
        assert_eq!(parse("3>=4"), Ok(0));
        assert_eq!(parse("1<2&&2!=3"), Ok(1));
    }

    #[test]
    fn evaluates_unary_operators() {
        assert_eq!(parse("-1"), Ok(u64::MAX));
        assert_eq!(parse("~0"), Ok(u64::MAX));
        assert_eq!(parse("!5"), Ok(0));
    }

    #[test]
    fn reports_errors() {
        assert!(parse("1/0").is_err());
        assert!(parse("1%0").is_err());
        assert!(parse("(1").is_err());
        assert!(parse("1)").is_err());
    }
}
