/// Parses a string representing an integer.
/// Works similarly to int(x, 0) in Python.
/// Negative values are returned in two's complement.
#[allow(clippy::option_if_let_else)]
pub fn parse_int(text: &str) -> Result<u64, String> {
    if let Some(x) = text.strip_prefix('-') {
        return parse_int(x).map(u64::wrapping_neg);
    }

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

#[cfg(test)]
mod tests {
    use super::parse_int;

    #[test]
    fn parses_radix_prefixes() {
        assert_eq!(parse_int("0x10"), Ok(16));
        assert_eq!(parse_int("0b1010"), Ok(10));
        assert_eq!(parse_int("0o10"), Ok(8));
        assert_eq!(parse_int("10"), Ok(10));
    }

    #[test]
    fn parses_negatives_as_twos_complement() {
        assert_eq!(parse_int("-1"), Ok(u64::MAX));
        assert_eq!(parse_int("-0b101"), Ok(u64::MAX - 4));
        assert_eq!(parse_int("-0"), Ok(0));
    }

    #[test]
    fn rejects_garbage() {
        assert!(parse_int("0xzz").is_err());
        assert!(parse_int("99999999999999999999").is_err());
    }
}
