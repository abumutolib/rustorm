#[derive(Debug)]
pub enum RustormError {
    Io(std::io::Error),
    Auth(String),
    Protocol(String),
    Query(String),
    Connection(String),
    Utf8(std::string::FromUtf8Error),
}

impl From<std::io::Error> for RustormError {
    fn from(err: std::io::Error) -> Self {
        RustormError::Io(err)
    }
}

impl From<std::string::FromUtf8Error> for RustormError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        RustormError::Utf8(err)
    }
}