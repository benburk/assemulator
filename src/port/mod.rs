mod state;
mod util;

pub use state::State;

/// The different ports.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Port {
    /// Print character to screen, read char from stdin
    Char,
    /// Print unsigned byte to screen, read byte from stdin
    Ticker,
}

impl TryFrom<usize> for Port {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Char),
            1 => Ok(Self::Ticker),

            _ => Err(format!("Invalid port address: {value}")),
        }
    }
}

impl From<Port> for usize {
    fn from(port: Port) -> Self {
        match port {
            Port::Char => 0,
            Port::Ticker => 1,
        }
    }
}
