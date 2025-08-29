#[derive(Debug)]
pub enum ConfigError {
    FileNotFoundError(String),
    ParseError(String),
    IOError(std::io::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::FileNotFoundError(path) => write!(f, "File not found: {}", path),
            ConfigError::ParseError(error) => write!(f, "Parse error: {}", error),
            ConfigError::IOError(error) => write!(f, "IO error: {}", error),
        }
    }
}

impl std::error::Error for ConfigError {}


