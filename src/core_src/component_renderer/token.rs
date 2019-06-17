
use crate::core_src::component_renderer::lexer::Lexer;
use crate::core_src::component_renderer::lexer::CodeBlock;

pub type TokenHook = fn(code_block: &mut CodeBlock) ->  &mut CodeBlock;

#[derive(Clone)]
pub enum TokenType {
    Keywords,
    Identifiers,
    Constants,
    Strings,
    SpecialSymbols,
    Operators,
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub label: String,
    pub callback: TokenHook
}

#[derive(Clone)]
pub struct TokenMatch {
    pub token: Token,
    pub value: String
}