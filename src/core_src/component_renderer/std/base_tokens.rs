use crate::core_src::component_renderer::token::{Token, TokenType};
use crate::core_src::component_renderer::lexer::{Lexer, CodeBlock};

pub fn register(input: &mut Vec<Token>) -> &mut Vec<Token> {

    // brace on
    input.push(
        Token {
            token_type: TokenType::SpecialSymbols, 
            label: String::from("{") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found brace on");
                return code_block;
            }
        }
    );

    // brace off
    input.push(
        Token {
            token_type: TokenType::SpecialSymbols, 
            label: String::from("}") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found brace off");
                return code_block;
            }
        }
    );

    // parenthese on
    input.push(
        Token {
            token_type: TokenType::SpecialSymbols, 
            label: String::from("(") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found parenthese on");
                return code_block;
            }
        }
    );

    // parenthese off
    input.push(
        Token {
            token_type: TokenType::SpecialSymbols, 
            label: String::from(")") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found parenthese off");
                return code_block;
            }
        }
    );

    // space
    input.push(
        Token {
            token_type: TokenType::SpecialSymbols, 
            label: String::from(" ") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found spaace");
                return code_block;
            }
        }
    );

    // eol
    input.push(
        Token {
            token_type: TokenType::SpecialSymbols, 
            label: String::from("\n") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found eol");
                return code_block;
            }
        }
    );

    // tab
    input.push(
        Token {
            token_type: TokenType::SpecialSymbols, 
            label: String::from("   ") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found eol");
                return code_block;
            }
        }
    );

    // quote
    input.push(
        Token {
            token_type: TokenType::SpecialSymbols, 
            label: String::from("\"") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found quote");
                return code_block;
            }
        }
    );

    // quote
    input.push(
        Token {
            token_type: TokenType::SpecialSymbols, 
            label: String::from("\"") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found quote");
                return code_block;            }
        }
    );

    // quote
    input.push(
        Token {
            token_type: TokenType::SpecialSymbols, 
            label: String::from("@") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found arobase");
                return code_block;
            }
        }
    );

    // quote
    input.push(
        Token {
            token_type: TokenType::SpecialSymbols, 
            label: String::from("@") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found arobase");
                return code_block;
            }
        }
    );

    // let
    input.push(
        Token {
            token_type: TokenType::Keywords, 
            label: String::from("let") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found let");
                return code_block;
            }
        }
    );

    // assignation
    input.push(
        Token {
            token_type: TokenType::Operators, 
            label: String::from(":=") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found assignation");
                return code_block;
            }
        }
    );

    // equality
    input.push(
        Token {
            token_type: TokenType::Operators, 
            label: String::from("=") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found equal");
                return code_block;
            }
        }
    );

    // division
    input.push(
        Token {
            token_type: TokenType::Operators, 
            label: String::from("/") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found division");
                return code_block;
            }
        }
    );

    // multiplication
    input.push(
        Token {
            token_type: TokenType::Operators, 
            label: String::from("*") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found multiplication");
                return code_block;
            }
        }
    );

    // multiplication
    input.push(
        Token {
            token_type: TokenType::Operators, 
            label: String::from("*") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found multiplication");
                return code_block;
            }
        }
    );

    // exp
    input.push(
        Token {
            token_type: TokenType::Operators, 
            label: String::from("_") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found exp");
                return code_block;
            }
        }
    );

    // injection
    input.push(
        Token {
            token_type: TokenType::Operators, 
            label: String::from("↣") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found injection");
                return code_block;
            }
        }
    );

    // surjection
    input.push(
        Token {
            token_type: TokenType::Operators, 
            label: String::from("↠") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found injection");
                return code_block;
            }
        }
    );

    // bijection
    input.push(
        Token {
            token_type: TokenType::Operators, 
            label: String::from("⇆") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found bijection");
                return code_block;
            }
        }
    );

    // relation
    input.push(
        Token {
            token_type: TokenType::Operators, 
            label: String::from("↦") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found relation");
                return code_block;
            }
        }
    );

    // relation
    input.push(
        Token {
            token_type: TokenType::Operators, 
            label: String::from("↦") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found relation");
                return code_block;
            }
        }
    );

    // non-linear vector 
    input.push(
        Token {
            token_type: TokenType::Operators, 
            label: String::from("↪") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found non-linear vector");
                return code_block;            }
        }
    );

    // linear vector 
    input.push(
        Token {
            token_type: TokenType::Operators, 
            label: String::from("↗") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found linear vector");
                return code_block;            }
        }
    );
   
    // linear vector 
    input.push(
        Token {
            token_type: TokenType::Operators, 
            label: String::from("∈") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found element of ");
                return code_block;            }
        }
    );

    // super division
    input.push(
        Token {
            token_type: TokenType::Operators, 
            label: String::from("__") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found super division");
                return code_block;            }
        }
    );

    // square
    input.push(
        Token {
            token_type: TokenType::Operators, 
            label: String::from("√") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found sqaure root");
                return code_block;            }
        }
    );

    // double dot
    input.push(
        Token {
            token_type: TokenType::Operators, 
            label: String::from(":") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found function declaration");
                return code_block;            }
        }
    );

    // declarations
    input.push(
        Token {
            token_type: TokenType::Operators, 
            label: String::from("declarations") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found object properties");
                return code_block;            }
        }
    );

    // double double dot
    input.push(
        Token {
            token_type: TokenType::Operators, 
            label: String::from("::") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found object args separator");
                return code_block;            }
        }
    );

    // action end
    input.push(
        Token {
            token_type: TokenType::Operators, 
            label: String::from(";") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found instruction delimiter");
                return code_block;            }
        }
    );

    // pi
    input.push(
        Token {
            token_type: TokenType::Operators, 
            label: String::from("π") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found π");
                return code_block;            }
        }
    );

    // coma
    input.push(
        Token {
            token_type: TokenType::Operators, 
            label: String::from(",") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found coma");
                return code_block;            }
        }
    );

    // coma
    input.push(
        Token {
            token_type: TokenType::Operators, 
            label: String::from(".") , 
            callback: |code_block: &mut CodeBlock| -> &mut CodeBlock {
                println!("Found dot");
                return code_block;            }
        }
    );


    return input;
}