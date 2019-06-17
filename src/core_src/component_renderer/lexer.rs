use crate::core_src::component_renderer::ast::Ast;
use crate::core_src::component_renderer::token::Token;

#[derive(Clone)]
pub struct CodeBlock  {
    pub left_side: String,
    pub right_side: String,
    pub current_word: String,
    pub lexer: Lexer
}

#[derive(Clone)]
pub struct Expectations {
    pub object_name: bool,
    pub braces_open: bool,
    pub object_states: bool,
    pub object_render: bool,
    pub object_functions: bool,
    pub function_arg_open_parenthese: bool,
    pub function_arg_close_parenthese: bool,
    pub function_arg: bool,
    pub arg_separator: bool,
    pub function_type_arrow: bool,
    pub function_type: bool,
    pub function_open_brace: bool,
    pub function_close_brace: bool,
    pub function_body: bool,
    pub value: bool,
    pub identifier: bool,
    pub operator: bool,
    pub variable_let_keyword: bool,
    pub html_start: bool,
    pub html_end: bool,
    pub arbobase_macro: bool,
    pub string_content: bool,
}

#[derive(Clone)]
pub struct Lexer {
    pub ast: Ast,
    pub root_dir: String,
    pub current_file: String,
    pub current_line: u64,
    pub buffer: Vec<CodeBlock>,
    pub int_buffer: String,
    pub string_buffer: String,
    pub label_buffer: String,
    pub quotes_open: bool,
    pub call_stack: Vec<Lexer>,
    pub parentheses_level: i16,
    pub braces_level: i16,
    pub current_scope: i16,
    pub bracket_level: i16,
    pub norm_level: i16,
    pub html_level: i16,
    pub expectations: Expectations
}