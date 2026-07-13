//! The lexical analysis

use num_bigint::BigUint;
use regex::Regex;
use std::str::FromStr;

/// A `weirdly_long_language_i_guess` keyword
#[derive(Debug, Clone)]
pub enum Keyword {
    /// A function declaration
    Function,
    /// A loop
    Loop,
}
/// The kind of a single `weirdly_long_language_i_guess` token
#[derive(Debug, Clone)]
pub enum TokenKind {
    // general
    /// An identifier
    Ident(String),
    /// A keyword
    Keyword(Keyword),

    // functions
    /// A left angle bracket `<`
    LeftAngle,
    /// A right angle bracket `>`
    RightAngle,
    /// A colon `:`
    Colon,
    /// A comma `,`
    Comma,

    // literals
    /// A string literal `"string"`
    String(String),
    /// An integer literal `69420`, `-42069`
    Number(BigUint),

    // statement delimeters
    /// A space (used for control flow delineation, like Python)
    Space,
    /// A statement terminator (a newline)
    Terminator,
}
/// A single `weirdly_long_language_i_guess` token
#[derive(Debug)]
pub struct Token {
    /// The kind of token
    pub kind: TokenKind,
    /// What line the token was taken from
    pub line: usize,
    /// What character of the line the token was taken from
    pub character: usize,
}

/// A stream of tokens
#[derive(Default, Debug)]
pub struct TokenStream(pub Vec<Token>);
impl TokenStream {
    /// Contructs a new, empty `TokenStream`
    #[must_use]
    pub fn new() -> TokenStream {
        Self(Vec::new())
    }
}
