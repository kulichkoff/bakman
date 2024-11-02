#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    Other(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(error) => write!(f, "IO Error: {}", error),
            Error::Other(error) => write!(f, "Error: {}", error),
        }
    }
}

impl std::error::Error for Error {}

impl Error {
    fn new(msg: &str) -> Self {
        Error::Other(msg.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}
