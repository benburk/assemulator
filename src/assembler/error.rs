use std::fmt;
use std::path::Path;

use pest::Position;
use pest::error::ErrorVariant;

use super::{Token, parser::Rule};

/// An assembly error, pointing at the source that caused it where possible.
#[derive(Debug)]
pub enum Error {
    /// An error at a location in a source file.
    Located(Box<pest::error::Error<Rule>>),
    /// An error without a location, or one that is already formatted.
    Message(String),
}

impl Error {
    /// An error spanning `pair`.
    pub fn at(pair: &Token, message: &str) -> Self {
        let variant = ErrorVariant::CustomError {
            message: message.to_owned(),
        };
        Self::Located(Box::new(pest::error::Error::new_from_span(
            variant,
            pair.as_span(),
        )))
    }

    /// An error at a single position.
    pub fn at_pos(pos: Position, message: &str) -> Self {
        let variant = ErrorVariant::CustomError {
            message: message.to_owned(),
        };
        Self::Located(Box::new(pest::error::Error::new_from_pos(variant, pos)))
    }

    /// Names the file the error is in, unless it's already named.
    #[must_use]
    pub fn in_file(self, path: &Path) -> Self {
        match self {
            Self::Located(err) if err.path().is_none() => {
                Self::Located(Box::new(err.with_path(&path.display().to_string())))
            }
            err => err,
        }
    }
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(err: pest::error::Error<Rule>) -> Self {
        Self::Located(Box::new(err))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Located(err) => err.fmt(f),
            Self::Message(message) => f.write_str(message),
        }
    }
}
