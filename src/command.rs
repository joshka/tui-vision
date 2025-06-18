use std::fmt::Display;
use std::str::FromStr;

/// A unique identifier for a command
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Command(String);

impl Command {
    /// Create a new command ID
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get the string representation of the command ID
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Command {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl From<String> for Command {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl FromStr for Command {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}
