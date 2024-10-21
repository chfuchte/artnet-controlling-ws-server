#[derive(Debug)]
pub enum ConfigParseError {
    YamlError(serde_yaml::Error),
    // fixture.channel binding
    BindingFixtureChannelDoesNotExist(String),
    /// fixture_type, fixture
    FixtureTypeNotFound(String, String),
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
        }
    }
}
