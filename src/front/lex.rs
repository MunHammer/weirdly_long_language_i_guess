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
impl super::SourceProgram {
    #[must_use]
    fn lex_single_char_token(&mut self, pos: &mut (usize, usize)) -> Option<Token> {
        let regexes: Vec<(Regex, TokenKind)> = vec![
            (
                Regex::new("^<").expect("Pattern should be correct"),
                TokenKind::LeftAngle,
            ),
            (
                Regex::new("^>").expect("Pattern should be correct"),
                TokenKind::RightAngle,
            ),
            (
                Regex::new("^:").expect("Pattern should be correct"),
                TokenKind::Colon,
            ),
            (
                Regex::new("^,").expect("Pattern should be correct"),
                TokenKind::Comma,
            ),
            (
                Regex::new("^ ").expect("Pattern should be correct"),
                TokenKind::Space,
            ),
            (
                Regex::new(r"^\n").expect("Pattern should be correct"),
                TokenKind::Terminator,
            ),
        ];
        for (regex, result) in &regexes {
            if let Some(match_found) = regex.find(&self.0) {
                self.0.drain(0..match_found.end());
                if let TokenKind::Terminator = result {
                    pos.1 = 0;
                    pos.0 += 1;
                }
                return Some(Token {
                    kind: result.clone(),
                    line: pos.0,
                    character: pos.1,
                });
            }
        }
        None
    }
    fn lex_keyword_token(&mut self, pos: &(usize, usize)) -> crate::Result<Option<(Token, usize)>> {
        let regexes = vec![
            (
                Regex::new(r"^[_A-Za-z]*[Ff]unction[_A-Za-z]*").expect("Pattern should be correct"),
                TokenKind::Keyword(Keyword::Function),
            ),
            (
                Regex::new(r"^[_A-Za-z]*[Ll]oop[_A-Za-z]*").expect("Pattern should be correct"),
                TokenKind::Keyword(Keyword::Loop),
            ),
        ];
        for (regex, result) in &regexes {
            if let Some(match_found) = regex.find(&self.0) {
                if match_found.end() < 20 {
                    return Err(crate::Error::new(crate::ErrorKind::Syntax, pos.0, pos.1));
                }
                return Ok(Some((
                    Token {
                        kind: result.clone(),
                        line: pos.0,
                        character: pos.1,
                    },
                    match_found.end(),
                )));
            }
        }
        Ok(None)
    }
    #[must_use]
    fn lex_identifier_token(&mut self, pos: &(usize, usize)) -> Option<(Token, usize)> {
        let regex = Regex::new(r"^[_A-Za-z]{19,}[_A-Za-z0-9]+").expect("Pattern should be correct");
        if let Some(match_found) = regex.find(&self.0) {
            let drained: String = self.0.drain(0..match_found.end()).collect();
            return Some((
                Token {
                    kind: TokenKind::Ident(drained.clone()),
                    line: pos.0,
                    character: pos.1,
                },
                drained.len(),
            ));
        }
        None
    }
    #[must_use]
    fn lex_integer(&mut self, pos: &(usize, usize)) -> Option<(Token, usize)> {
        let integer = Regex::new(r"^[0-9]+").expect("Pattern should be correct");
        if let Some(match_found) = integer.find(&self.0) {
            let drained = self.0.drain(0..match_found.end()).collect::<String>();
            return Some((
                Token {
                    kind: TokenKind::Number(
                        BigUint::from_str(&drained)
                            .expect("Shouldn't error, it has already been regex parsed"),
                    ),
                    line: pos.0,
                    character: pos.1,
                },
                drained.len(),
            ));
        }
        None
    }
    /// Performs lexical analysis on the source code
    /// # Errors
    /// None
    /// # Panics
    /// If the regex pattern has syntax errors
    pub fn lex(mut self) -> crate::Result<TokenStream> {
        let mut stream = TokenStream::new();
        // let code = &mut self.0;
        let mut pos: (usize, usize) = (1, 1);
        let mut passed: bool;
        loop {
            passed = false;
            if let Some(token) = self.lex_single_char_token(&mut pos) {
                stream.0.push(token);
                pos.1 += 1;
                passed = true;
            } else if let Some((token, length)) = self.lex_keyword_token(&pos)? {
                stream.0.push(token);
                pos.1 += self.0.drain(0..length).collect::<String>().len();
                passed = true;
            } else if let Some((token, length)) = self.lex_identifier_token(&pos) {
                stream.0.push(token);
                pos.1 = length;
                passed = true;
            } else if let Some((token, length)) = self.lex_integer(&pos) {
                stream.0.push(token);
                pos.1 += length;
                passed = true;
            }
            if !passed {
                return Err(crate::Error::new(crate::ErrorKind::Syntax, pos.0, pos.1));
            }
            if self.0.is_empty() {
                break;
            }
        }
        Ok(stream)
    }
}
