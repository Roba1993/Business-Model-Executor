pub type Result<T> = std::result::Result<T, crate::error::Error>;

#[derive(Debug)]
pub struct Error {
    error: std::result::Result<String, Box<dyn std::error::Error>>,
}

impl Error {
    pub fn new(error: Box<dyn std::error::Error>) -> Error {
        Error { error: Err(error) }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.error {
            Ok(s) => write!(f, "{}", s),
            Err(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match &self.error {
            Ok(s) => s,
            Err(e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}

// immplement error from string
impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error {
            error: Ok(err.to_string()),
        }
    }
}

// immplement error for system time
impl From<std::time::SystemTimeError> for Error {
    fn from(err: std::time::SystemTimeError) -> Self {
        Error::new(Box::new(err))
    }
}

// immplement error for system time
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::new(Box::new(err))
    }
}