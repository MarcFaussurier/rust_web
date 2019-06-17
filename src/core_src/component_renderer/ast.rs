use crate::core_src::component_renderer::token::TokenMatch;
use crate::core_src::component_renderer::memory::MemoryCell;

#[derive(Clone)]
pub struct Ast {
    pub token_list: Vec<TokenMatch>,
    pub memory: Vec<MemoryCell>
}