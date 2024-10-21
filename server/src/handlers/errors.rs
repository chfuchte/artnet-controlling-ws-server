#[derive(Debug)]
pub enum WebsocketHandlingError {
    IoError(std::io::Error),
    UnknownMessage(String),
    DfcFixtureNotFound(String),
    DfcChannelNotFound(String),
    VariableNotFound(String),
    VariableParseError(String),
}

impl WebsocketHandlingError {}

impl From<std::io::Error> for WebsocketHandlingError {
    fn from(err: std::io::Error) -> Self {
        WebsocketHandlingError::IoError(err)
    }
}
