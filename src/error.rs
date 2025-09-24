use std::fmt;
use std::io;

#[derive(Debug)]
pub enum BingrepError {
    Io(io::Error),
    InvalidPattern(String),
    InvalidWidth(usize),
    RegexCompilation(String),
    GlobPattern(String),
    GlobPath(String),
}

impl fmt::Display for BingrepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BingrepError::Io(err) => write!(f, "IO error: {}", err),
            BingrepError::InvalidPattern(msg) => write!(f, "Invalid pattern: {}", msg),
            BingrepError::InvalidWidth(width) => {
                let config = crate::Config::default();
                write!(
                    f,
                    "Invalid width {}: must be between {} and {}",
                    width,
                    config.get_min_width(),
                    config.get_max_width()
                )
            }
            BingrepError::RegexCompilation(msg) => write!(f, "Regex compilation error: {}", msg),
            BingrepError::GlobPattern(msg) => write!(f, "Glob pattern error: {}", msg),
            BingrepError::GlobPath(msg) => write!(f, "Glob path error: {}", msg),
        }
    }
}

impl std::error::Error for BingrepError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BingrepError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for BingrepError {
    fn from(err: io::Error) -> Self {
        BingrepError::Io(err)
    }
}

impl From<regex::Error> for BingrepError {
    fn from(err: regex::Error) -> Self {
        BingrepError::RegexCompilation(err.to_string())
    }
}

impl From<glob::PatternError> for BingrepError {
    fn from(err: glob::PatternError) -> Self {
        BingrepError::GlobPattern(err.to_string())
    }
}

impl From<glob::GlobError> for BingrepError {
    fn from(err: glob::GlobError) -> Self {
        BingrepError::GlobPath(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, BingrepError>;
