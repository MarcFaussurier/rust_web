use crate::core_src::component_renderer::lexer::{Lexer, Expectations, CodeBlock};
use crate::core_src::component_renderer::token::{Token, TokenHook, TokenMatch, TokenType};
use crate::core_src::component_renderer::memory::{MemoryCell};
use crate::core_src::component_renderer::ast::Ast;

use std::fs;
use std::str::Chars;
// tokens 
use crate::core_src::component_renderer::std::base_tokens::register;


pub fn instanciate_lexer(root_dir: String, file_path_ref: String) -> Lexer {
    return Lexer 
    {
        ast: Ast{
            token_list: vec![],
            memory: vec![]
        },
        root_dir: root_dir,
        string_buffer: String::from(""),
        label_buffer: String::from(""),
        current_file: file_path_ref,
        current_line: 1,
        buffer: vec![],
        int_buffer: String::from(""),
        quotes_open: false,
        call_stack: vec![],
        parentheses_level: 0,
        braces_level: 0,
        current_scope: 0,
        bracket_level: 0,
        norm_level: 0,
        html_level: 0,
        expectations: Expectations {
            object_name: false,
            braces_open: false,
            object_states: false,
            object_render: false,
            object_functions: false,
            function_arg_open_parenthese: false,
            function_arg_close_parenthese: false,
            function_arg: false,
            arg_separator: false,
            function_type_arrow: false,
            function_type: false,
            function_open_brace: false,
            function_close_brace: false,
            function_body: false,
            value: false,
            identifier: false,
            operator: false,
            variable_let_keyword: false,
            html_start: false,
            html_end: false,
            arbobase_macro: false,
            string_content: false,
     }};
}

pub fn insert_token_of_word_into(code_block: CodeBlock, word: String, tokens: &mut Vec<Token>) -> TokenMatch {
    let mut output: TokenMatch = TokenMatch{

        
        token: Token {
            token_type: TokenType::Keywords,
            label: String::from("EMPTY"),
            callback: |code_block: &mut CodeBlock| ->  &mut CodeBlock {
                return code_block;
            }
        },
        value: String::from("")
    };
    
    for x in tokens.iter() {
        output.token = x.clone();
    }

    return output;
}

pub fn parse_chars(mut code_block: CodeBlock, chars: Chars, tokens: &mut Vec<Token>) -> CodeBlock {
   /* for c in chars {

        // if reading a string => append it 

        // if reading a int => append it 

        // if reading identifier => append it

        // foreach tokens, if match => execute it 

        let clonned = code_block.clone();
        code_block.current_word = format!("{}{}", clonned.current_word, c);

        // the function will probably call itself

        let clonned = code_block.clone();
        insert_token_of_word_into(code_block,  clonned.clone().current_word, tokens);

    }*/
    return code_block;
}

pub fn parse_source_code(file_path: &String) {
    let root_path = String::from("./../../src/app_src/common/components/");
    let path = String::from(root_path.clone()) + file_path;
    println!("path {}", path);

    let file_path_ref = file_path.clone();
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");


    let mut tokens: Vec<Token> = vec![];
    // here we add std base tokens
    let tokens = register(&mut tokens);
    println!("token size :  {}", tokens.len());

    let lexer = instanciate_lexer(root_path, file_path_ref);

    let mut code_block = CodeBlock {
        left_side: String::from(""), 
        current_word: String::from(""), 
        right_side: String::from(""),
        lexer: lexer
    };

   // parse_chars(code_block, contents.chars(), tokens);

    println!("With text:\n{}", contents);
}