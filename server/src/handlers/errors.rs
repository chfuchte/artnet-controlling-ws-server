#[derive(Debug)]
pub enum WebsocketHandlingError {
    /// error from commiting by the ArtNetClient
    /// arg: std::io::Error from ArtNetClient.commit() method
    IoError(std::io::Error),

    /// no binding found for the given fixture
    /// arg: original message
    UnknownMessage(String),

    /// no fixture found for the given name
    /// arg: fixture trying to be found
    FixtureNotFound(String),

    // no channel found for the given name
    /// arg1: fixture name found
    /// arg2: channel name which could not be found
    ChannelNotFound(String, String),

    /// error from parsing a value (str) to u8
    /// arg: std::num::ParseIntError from calling str.parse
    ParseVariableToNumError(std::num::ParseIntError),

    /// error when an incomming message is in DFC format but DFC is disabled in the config
    DfcDisabledButMsgIsInDfcFormat,

    /// error extracting variables from a message
    /// arg: the value string no variable could be found for
    ExtractVariableError(String),
}

impl WebsocketHandlingError {}

impl From<std::io::Error> for WebsocketHandlingError {
    fn from(err: std::io::Error) -> Self {
        WebsocketHandlingError::IoError(err)
    }
}
