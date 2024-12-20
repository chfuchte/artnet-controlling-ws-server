#[derive(Debug)]
pub enum ConfigParseError {
    YamlError(serde_yaml::Error),
    // fixture.channel binding
    BindingFixtureChannelDoesNotExist(String),
    /// fixture_type, fixture
    FixtureTypeNotFound(String, String),
    InvalidActionOrStepsBinding,
    /// mode
    InvalidStepsMode(String),
}

impl ToString for ConfigParseError {
    fn to_string(&self) -> String {
        match self {
            ConfigParseError::YamlError(e) => format!("error parsing yaml: {}", e),
            ConfigParseError::BindingFixtureChannelDoesNotExist(s) => {
                format!("fixture.channel '{}' pair in bindings does not exist", s)
            }
            ConfigParseError::FixtureTypeNotFound(fixture_type, fixture) => {
                format!(
                    "fixture type {} does not exists at fixture {}",
                    fixture_type, fixture
                )
            }
            ConfigParseError::InvalidActionOrStepsBinding => {
                format!("invalid binding: in action mode neither mode or steps are not allowed and in steps mode action is not allowed")
            }
            ConfigParseError::InvalidStepsMode(mode) => {
                format!("{} is not a valid mode. use alternate or once", mode)
            }
        }
    }
}
