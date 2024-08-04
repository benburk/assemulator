use strum::ParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Register<const N: usize>(pub usize);

impl<const N: usize> TryFrom<&str> for Register<N> {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut chars = value.chars();

        if chars.next() != Some('r') {
            return Err(ParseError::VariantNotFound);
        }

        let value = chars
            .as_str()
            .parse::<usize>()
            .map_err(|_| ParseError::VariantNotFound)?;

        if value >= N {
            return Err(ParseError::VariantNotFound);
        }
        Ok(Self(value))
    }
}
