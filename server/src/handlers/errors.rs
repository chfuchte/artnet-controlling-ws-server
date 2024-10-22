use std::num::ParseIntError;

#[derive(Debug)]
pub enum WebsocketHandlingError {
    IoError(std::io::Error),
    UnknownMessage(String),
    FixtureNotFound(String),
    InvalidActionOrDfcFormat(String),
    ChannelNotFound(String),
    VariableNotFound(String),
    VariableParseError(ParseVariableError),
    DfcDisabledButMsgIsInDfcFormat,
}

impl WebsocketHandlingError {}

impl From<std::io::Error> for WebsocketHandlingError {
    fn from(err: std::io::Error) -> Self {
        WebsocketHandlingError::IoError(err)
    }
}

#[derive(Debug)]
pub struct ParseVariableError {
    details: String,
}

impl ParseVariableError {
    fn new(msg: &str) -> ParseVariableError {
        ParseVariableError {
            details: msg.to_string(),
        }
    }

    pub fn get_details(&self) -> &str {
        &self.details
    }
}

impl From<ParseIntError> for ParseVariableError {
    fn from(_: ParseIntError) -> Self {
        ParseVariableError::new("Failed to parse variable value as u8")
    }
}
