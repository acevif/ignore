use crate::domain::Ignorefile;
use std::io::Read;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IgnorefileParseError {
    message: String,
}

impl IgnorefileParseError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for IgnorefileParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ignorefile parse error: {}", self.message)
    }
}

impl std::error::Error for IgnorefileParseError {}

pub trait IgnorefileParser {
    fn parse<R: Read>(&self, reader: R) -> Result<Ignorefile, IgnorefileParseError>;
}

pub fn parse_ignorefile<P: IgnorefileParser, R: Read>(
    parser: &P,
    reader: R,
) -> Result<Ignorefile, IgnorefileParseError> {
    parser.parse(reader)
}
