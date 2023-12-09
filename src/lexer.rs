#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(dead_code)]

extern crate exitcode;

use crate::token::Token;
use crate::token::get_token;
use std::collections::HashSet;

#[derive(Debug)]
pub struct MyLexer {
    input_string: String,
    input_position: usize,
    input_current_state: usize,
    input_current_token: Token,
    buffer_string: String,
}

impl MyLexer {
    pub fn set_input(string: &str) -> MyLexer {
        MyLexer {
            input_string: string.parse().unwrap(),
            input_position: 0,
            input_current_state: 0,
            input_current_token: Token::SOI,
            buffer_string: "".to_string(),
        }
    }

    fn print_tokens(&mut self) {
        let mut token = self.curr();
        print!("{:?} ", token);
        while token != Token::EOI {
            self.advance();
            token = self.curr();
            print!("{:?} ", token);
        }
        println!();
    }

    pub(crate) fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec!();

        self.advance();
        let mut token = self.curr();

        while token != Token::EOI {
            tokens.push(token.clone());
            self.advance();
            token = self.curr();
        }
        tokens
    }


    fn curr(&self) -> Token {
        return self.input_current_token.clone();
    }

    fn advance(&mut self) {
        // Create hardcoded keywords to check.
        let keywords: HashSet<&str> = HashSet::from(["func", "let", "if", "else", "while", "print", "i32", "f32", "char", "return", "bool", "false", "true"]);

        if self.input_current_token == Token::EOI {
            println!();
            eprintln!("ERROR: End of Input\n\tToken:{:?}\n\tCannot proceed to use this function call.", Token::EOI);
            std::process::exit(exitcode::OK);
        }

        let chars = self.input_string.chars();
        let mut run = false;

        self.buffer_string = "".to_string();
        self.input_current_state = 0;

        while !run {
            //run == true means accepting state.

            //End of Input - Buffer String Is Full
            if chars.clone().nth(self.input_position).is_none() && self.buffer_string != "" {
                match self.input_current_state {
                    7 => {
                        //Keyword | ID
                        for keyword in keywords.clone() {
                            if self.buffer_string == keyword {
                                if self.buffer_string == "false" || self.buffer_string == "true" {
                                    self.input_current_token = Token::LIT_BOOL(self.buffer_string.parse::<bool>().unwrap());
                                    run = true;
                                    break;
                                } else {
                                    self.input_current_token = get_token(&self.buffer_string);
                                    run = true;
                                    break;
                                }
                            }
                        }
                        if run != true {
                            self.input_current_token = Token::ID(String::from(&self.buffer_string));
                            run = true;
                        }
                    }
                    8 => {
                        self.input_current_token = Token::LIT_I32(self.buffer_string.parse().unwrap());
                        run = true;
                    }
                    10 => {
                        //It is a f32
                        self.input_current_token = Token::LIT_F32(self.buffer_string.parse().unwrap());
                        run = true;
                    }
                    16 => {
                        //It's a semi string lit
                        self.buffer_string.remove(0);
                        println!();
                        eprintln!("ERROR: String Literal\n\tToken:{:?}\n\tNo closing of String Literal.", Token::LIT_STRING(String::from(&self.buffer_string)));
                        std::process::exit(exitcode::OK);
                    }
                    17 => {
                        //It's a semi char lit
                        self.buffer_string.remove(0);
                        println!();
                        eprintln!("ERROR: Char Literal\n\tToken:{:?}\n\tNo closing of Char Literal.", Token::LIT_CHAR(self.buffer_string.chars().clone().nth(0).unwrap()));
                        std::process::exit(exitcode::OK);
                    }
                    _ => {}
                }
            }

            //End of Input - Buffer String is Empty
            else if chars.clone().nth(self.input_position).is_none() && self.buffer_string == "" {
                self.input_current_token = Token::EOI;
                run = true;
            }


            /* Move to inside a part...
            */

            //WhiteSpace---Skip

            else if chars.clone().nth(self.input_position).unwrap().is_whitespace() && self.buffer_string == "" {
                self.input_position += 1;
            }
            else {

                //Check the char before we push to string.
                let char = chars.clone().nth(self.input_position).unwrap();
                if self.buffer_string.len() > 0 {
                    match self.input_current_state {
                        0 | 1 => {
                            println!();
                            eprintln!("ERROR: Input State {}... Should not be here.", self.input_current_state);
                            std::process::exit(exitcode::OK);
                        }
                        2 => {
                            //. State
                            if char.is_digit(10) && !char.is_alphabetic() {
                                self.input_current_state = 10;
                            } else {
                                self.input_current_token = get_token(&self.buffer_string);
                                run = true;
                            }
                        }
                        3 => {
                            //- or ->
                            if char == '>' {
                                self.input_current_state = 11;
                            } else {
                                self.input_current_token = get_token(&self.buffer_string);
                                run = true;
                            }
                        }
                        4 => {
                            //= or ==
                            if char == '=' {
                                self.input_current_state = 12;
                            } else {
                                self.input_current_token = get_token(&self.buffer_string);
                                run = true;
                            }
                        }
                        5 => {
                            // ! or != or !< or !>
                            if char == '=' || char == '<' || char == '>' {
                                self.input_current_state = 13;
                            }
                            else {
                                self.input_current_token = get_token(&self.buffer_string);
                                run = true;
                            }
                        }
                        6 => {
                            if char == '=' {
                                self.input_current_state = 14;
                            }
                            else {
                                self.input_current_token = get_token(&self.buffer_string);
                                run = true;
                            }
                        }
                        7 => {
                            //Keyword | ID
                            if !char.is_alphanumeric() && char != '_' {
                                for keyword in keywords.clone() {
                                    if self.buffer_string == keyword {
                                        if self.buffer_string == "false" || self.buffer_string == "true" {
                                            self.input_current_token = Token::LIT_BOOL(self.buffer_string.parse::<bool>().unwrap());
                                            run = true;
                                            break;
                                        } else {
                                            self.input_current_token = get_token(&self.buffer_string);
                                            run = true;
                                            break;
                                        }
                                    }
                                }
                                if run != true {
                                    self.input_current_token = Token::ID(String::from(&self.buffer_string));
                                    run = true;
                                }
                            }
                        }
                        8 => {
                            //LIT_INT or LIT_FLT
                            if char == '.' {
                                //Put it into LIT_FLT because it actually makes sense.
                                self.input_current_state = 10;
                            }
                            else if char.is_alphabetic() || !char.is_digit(10) {
                                self.input_current_token = Token::LIT_I32(self.buffer_string.parse().unwrap());
                                run = true;
                            }
                        }
                        9 => {
                            // & or && or | or ||
                            if char == '&' || char == '|' {
                                self.input_current_state = 15;
                            } else {
                                self.input_current_token = get_token(&self.buffer_string);
                                run = true;
                            }
                        }
                        10 => {
                            //It is a f32
                            if char.is_alphabetic() || !char.is_digit(10) {
                                self.input_current_token = Token::LIT_F32(self.buffer_string.parse().unwrap());
                                run = true;
                            }
                        }

                        // I realize I could put these all as one, but in terms of state machines, seems to get cluttered with many going to one state.
                        11 => {
                            // -> ARROW_R
                            self.input_current_token = get_token(&self.buffer_string);
                            run = true;
                        }
                        12 => {
                            // == EQ
                            self.input_current_token = get_token(&self.buffer_string);
                            run = true;
                        }
                        13 => {
                            // != NEQ, !< NLT, !> NGT
                            self.input_current_token = get_token(&self.buffer_string);
                            run = true;
                        }
                        14 => {
                            // <= NGT, >= NLT
                            self.input_current_token = get_token(&self.buffer_string);
                            run = true;
                        }
                        15 => {
                            // && AND, || OR
                            self.input_current_token = get_token(&self.buffer_string);
                            run = true;
                        }
                        16 => {
                            //String lit
                            if char == '"' {
                                self.input_current_state = 18;
                            }
                        }
                        17 => {
                            //char lit
                            if char == '\'' {
                                self.input_current_state = 19;
                            }
                            else if self.buffer_string.len() == 2 && char != '\'' {
                                self.buffer_string.remove(0);
                                println!();
                                eprintln!("ERROR: Char Literal\n\tToken:{:?}\n\tNo closing of Char Literal.", Token::LIT_CHAR(self.buffer_string.chars().clone().nth(0).unwrap()));
                                std::process::exit(exitcode::OK);
                            }
                        }
                        18 => {
                            self.buffer_string.remove(self.buffer_string.len()-1);
                            self.buffer_string.remove(0);

                            self.input_current_token = Token::LIT_STRING(String::from(&self.buffer_string));
                            run = true;
                        }
                        19 => {
                            self.buffer_string.remove(self.buffer_string.len()-1);
                            self.buffer_string.remove(0);

                            self.input_current_token =  Token::LIT_CHAR(self.buffer_string.chars().clone().nth(0).unwrap());
                            run = true;
                        }
                        _ => {}
                    }
                }


                //Add next character onto buffer.
                if !run {
                    self.buffer_string.push(char);
                    self.input_position += 1;
                }

                if self.buffer_string.len() == 1 && !run {
                    match char {
                        '(' | ')' | '[' | ']' | '{' | '}' | ',' | ':' | ';' | '+' | '*' | '/'  => {
                            self.input_current_token = get_token(&char.to_string());
                            self.input_current_state = 1;
                            run = true;
                        }
                        '.' => {
                            //Check for LIT_FLT32 or POINT
                            self.input_current_state = 2;
                        }
                        '-' => {
                            //Check for SUB or ARROW_R
                            self.input_current_state = 3;
                        }
                        '=' => {
                            //Check for ASSIGN or EQ
                            self.input_current_state = 4;
                        }
                        '!' => {
                            //Check for NOT, NEQ, NLT, NGT
                            self.input_current_state = 5;
                        }
                        '<' | '>' => {
                            //Check for LT, GT, NLT(GTE), NGT(LTE)
                            self.input_current_state = 6;
                        }
                        'A'..='Z' | 'a'..='z' | '_' => {
                            //Check for ID or Keyword
                            self.input_current_state = 7;
                        }
                        '0'..='9' => {
                            //Check for LIT_INT or LIT_FLT32
                            self.input_current_state = 8;
                        }
                        '&' | '|' => {
                            //Check for OR or AND or BIT_OR or BIT_AND
                            self.input_current_state = 9;
                        }
                        '\"' => {
                            //Check string literal... end at end of input or second (")
                            self.input_current_state = 16;
                        }
                        '\'' => {
                            //Check char literal... end at end of input or second(')
                            self.input_current_state = 17;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

