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
