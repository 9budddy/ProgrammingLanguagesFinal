#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::mem::discriminant;

#[derive(Debug, Clone)]
pub enum Token {

    // general
    SOI,
    EOI,
    ERROR,

    // atoms
    ID(String),
    LIT_I32(i32),
    LIT_F32(f32),
    LIT_CHAR(char),
    LIT_STRING(String),
    LIT_BOOL(bool),

    // types
    TYPE_I32, TYPE_F32, TYPE_CHAR, TYPE_BOOL,

    // arithmetic operators
    OP_ADD,
    OP_SUB,
    OP_MUL,
    OP_DIV,

    // relational operators
    OP_LT,          // less than
    OP_GT,          // greater than
    OP_NOT_LT,      // not less than == greater than or equal
    OP_NOT_GT,      // not greater than == less than or equal
    OP_EQUAL,       // equal
    OP_NOT_EQUAL,   // not equal

    // logical operators
    OP_NOT,
    OP_AND,
    OP_OR,
    OP_AND_BIT,
    OP_OR_BIT,

    // other operators
    OP_ASSIGN,

    // nesting
    PAREN_L,
    PAREN_R,
    BRACKET_L,
    BRACKET_R,
    BRACE_L,
    BRACE_R,

    // separators
    POINT,
    COMMA,
    COLON,
    SEMICOLON,
    ARROW_R,

    // keywords
    KW_FUNC,
    KW_LET,
    KW_IF,
    KW_ELSE,
    KW_WHILE,
    KW_PRINT,
    KW_RETURN,

    // calls
    CALLS(Vec<Token>),
}


impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Eq for Token { }

impl Token {
    pub fn id() -> Token {
        Token::ID(String::new())
    }
    pub fn lit_i32() -> Token {
        Token::LIT_I32(0)
    }
    pub fn lit_f32() -> Token {
        Token::LIT_F32(0.0)
    }
    pub fn lit_char() -> Token {
        Token::LIT_CHAR(' ')
    }
    pub fn lit_string() -> Token {
        Token::LIT_STRING(String::new())
    }
    pub fn lit_bool() -> Token {
        Token::LIT_BOOL(false)
    }
}

pub fn get_token(string: &str) -> Token {
    return match string {
        "(" => {
            Token::PAREN_L
        }
        ")" => {
            Token::PAREN_R
        }
        "[" => {
            Token::BRACKET_L
        }
        "]" => {
            Token::BRACKET_R
        }
        "{" => {
            Token::BRACE_L
        }
        "}" => {
            Token::BRACE_R
        }
        ":" => {
            Token::COLON
        }
        ";" => {
            Token::SEMICOLON
        }
        "," => {
            Token::COMMA
        }
        "+" => {
            Token::OP_ADD
        }
        "*" => {
            Token::OP_MUL
        }
        "/" => {
            Token::OP_DIV
        }
        "." => {
            Token::POINT
        }
        "-" => {
            Token::OP_SUB
        }
        "->" => {
            Token::ARROW_R
        }
        "=" => {
            Token::OP_ASSIGN
        }
        "==" => {
            Token::OP_EQUAL
        }
        "!" => {
            Token::OP_NOT
        }
        "!=" => {
            Token::OP_NOT_EQUAL
        }
        "!<" | ">=" => {
            Token::OP_NOT_LT
        }
        "!>" | "<=" => {
            Token::OP_NOT_GT
        }
        "<" => {
            Token::OP_LT
        }
        ">" => {
            Token::OP_GT
        }
        "&" => {
            Token::OP_AND_BIT
        }
        "&&" => {
            Token::OP_AND
        }
        "|" => {
            Token::OP_OR_BIT
        }
        "||" => {
            Token::OP_OR
        }
        "func" => {
            Token::KW_FUNC
        }
        "let" => {
            Token::KW_LET
        }
        "if" => {
            Token::KW_IF
        }
        "else" => {
            Token::KW_ELSE
        }
        "while" => {
            Token::KW_WHILE
        }
        "print" => {
            Token::KW_PRINT
        }
        "return" => {
            Token::KW_RETURN
        }
        "i32" => {
            Token::TYPE_I32
        }
        "f32" => {
            Token::TYPE_F32
        }
        "char" => {
            Token::TYPE_CHAR
        }
        "bool" => {
            Token::TYPE_BOOL
        }


        _ => { Token::EOI }
    }
}