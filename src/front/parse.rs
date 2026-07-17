use super::TokenStream;
use crate::front::lex::Token;
use crate::{Error, ErrorKind};
use std::collections::HashMap;

pub enum Node {
    FunctionDefinition {
        identifier: String,
        arguments: HashMap<String, crate::Type>,
        body: Block,
    },
    FunctionCall {
        identifier: String,
        arguments: HashMap<String, Block>,
    },
    Loop(Block),
}
#[derive(Default)]
pub struct Block(pub Vec<Node>);
impl Block {
    /// Creates a new `Block`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
/// expect_tokens!(ident; ident; $(pat $(=> ident)? = ident: ty $(= expr)),*$(,)? => expr)
macro_rules! expect_tokens {
    ($stream:ident; $base_token:ident; $($matcher:pat $(=> $re_name:ident)? = $returnee:ident: $re_ty:ty = $re_place:expr),*$(,)? => $thing:expr) => {
    // C: Fn(&TokenKind) -> Option<U>,
    // F: Fn(&mut I, U, &'a Token) -> T,
    $(
        let mut $returnee: crate::Result<$re_ty> = Ok($re_place);
        let token = $stream.next();
        if let Some(token) = token {
            if let $matcher = token.kind.clone() {
            $($returnee = Ok($re_name);)?
            } else {
                $returnee = Err(Error::new(ErrorKind::Syntax, token.line, token.character));
            }
            $base_token = token.clone();
        } else {
            $returnee = Err(Error::new(
                ErrorKind::Syntax,
                $base_token.line,
                $base_token.character,
            ));
        }
    )*
    $thing
    };
}
pub fn parse_section<'a, I: Iterator<Item = &'a Token>>(
    stream: &'a mut I,
    indentation: usize,
) -> Block {
    let mut output = Block::new();
    todo!()
}
impl TokenStream {
    #[must_use]
    pub fn parse(self) -> crate::Result<Block> {
        todo!()
    }
}
