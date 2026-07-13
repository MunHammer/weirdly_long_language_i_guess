//! The file for the language's error type

/// The Error type for `weirdly_long_language_i_guess`
#[derive(Debug)]
pub enum ErrorKind {
    /// A syntax error
    Syntax,
    /// An I/O error
    IO(std::io::ErrorKind),
}
/// The struct in which other information about the error is held
#[derive(Debug)]
pub struct Error {
    /// The type of error
    pub kind: ErrorKind,
    /// The line at which the error occured
    line: usize,
    /// The character at which the error occured
    character: usize,
}
/// A convenience type alias
pub type Result<T> = std::result::Result<T, Error>;
impl From<std::io::Error> for ErrorKind {
    fn from(error: std::io::Error) -> Self {
        Self::IO(error.kind())
    }
}
impl From<regex::Error> for ErrorKind {
    fn from(error: regex::Error) -> Self {
        dbg!(error);
        Self::Syntax
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self {
            kind: ErrorKind::from(error),
            line: 0,
            character: 0,
        }
    }
}

impl Error {
    /// Creates a new `LongLanguageError`
    #[must_use]
    pub fn new(kind: ErrorKind, line: usize, character: usize) -> Self {
        Self {
            kind,
            line,
            character,
        }
    }
}
