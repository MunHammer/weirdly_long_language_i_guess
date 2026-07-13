//! The frontend of the compiler
pub mod lex;
/// The source code of a program
pub struct SourceProgram(pub String);
impl SourceProgram {
    /// Constructs a new `SourceProgram`
    #[must_use]
    pub fn new(inner: String) -> Self {
        Self(inner)
    }
}
