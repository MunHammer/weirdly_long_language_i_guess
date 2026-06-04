//! The file for the language's error type

/// The Error type for `weirdly_long_language_i_guess`
#[derive(Debug)]
pub enum LongLanguageError {
    /// A syntax error
    Syntax,
    /// An I/O error
    IO(std::io::ErrorKind),
}
/// A convenience type alias
pub type LongLanguageResult<T> = Result<T, LongLanguageError>;
impl From<std::io::Error> for LongLanguageError {
    fn from(error: std::io::Error) -> Self {
        Self::IO(error.kind())
    }
}
