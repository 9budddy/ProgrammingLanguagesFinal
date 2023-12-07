#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::lexer_mockup::Lexer;
use crate::lexer::MyLexer;
use crate::pratt_parsing::brad_pratt;
use crate::token::Token;

const INDENT : usize = 2;


pub fn boolS() {
    let test2 =
"let global;
global = 5;

func factorial_recursion(n)
{
    if n < 2 {
        return 1;
    } else {
        return n;
    }
}
func factorial_loop(n)
{
    let p;
    p = n;
    while n > 0 {
        n = n - 1;
        p = p * n;
    }
    return p;
}
func main(argc)
{
    let n;
    n = 5;
    print factorial_loop(n);
    print factorial_recursion(n);
}
";

    let mut lexer;
    lexer = MyLexer::set_input(test2);

    let mut tokens = lexer.get_tokens();
    println!("{:?}", tokens);

    let lexer = Lexer::new(tokens);
    let mut parser = DescentParser::new(lexer);

    parser.analyze();

}


struct DescentParser {
    lexer: Lexer,
    indent: usize,
}


impl DescentParser {  // simple recursive descend parser

    fn new(lexer: Lexer) -> DescentParser {
        DescentParser {
            lexer,
            indent: 0,
        }
    }

    pub fn analyze(&mut self) {
        self.indent = 0;
        while !self.peek(Token::EOI) {
            if self.peek(Token::KW_FUNC) {
                self.parse_func();
            }
            else if self.peek(Token::KW_LET) {
                self.parse_let();
            }
            else if self.peek(Token::id()) {
                self.parse_assignment();
            }
        }
        self.expect(Token::EOI);
    }

    fn parse_func(&mut self) {
        /*
            Func -> func id List ("->" id)* Body
            List -> '(' (Param(, Param)*)? ')'
            Param -> id ':' id
            Body -> {(Let)* (If)* (Return)*}
        */

        self.indent_print("parse_func()");
        self.indent_increment();
        {
            self.expect(Token::KW_FUNC);
            self.expect(Token::id());
            self.parse_parameter_list();
            if self.peek(Token::ARROW_R) {
                self.expect(Token::ARROW_R);
                self.type_check();
            }
            self.parse_brace_nest();
        }
        self.indent_decrement();
    }

    fn parse_parameter_list(&mut self) {
        self.indent_print("parse_parameter_list()");
        self.indent_increment();
        {
            self.expect(Token::PAREN_L);
            if self.accept(Token::PAREN_R) {
                return;
            }
            self.parse_parameter();
            while self.accept(Token::COMMA) {
                self.parse_parameter();
            }
            self.expect(Token::PAREN_R);
        }
        self.indent_decrement();
    }

    fn parse_parameter(&mut self) {
        self.indent_print("parse_parameter()");
        self.indent_increment();
        {
            self.expect(Token::id());
            self.expect(Token::COLON);
            self.type_check();
        }
        self.indent_decrement();
    }

    fn type_check(&mut self) {
        if self.peek(Token::TYPE_I32) {
            self.expect(Token::TYPE_I32);
        }
        // else if self.peek(Token::TYPE_F32) {
        //     self.expect(Token::TYPE_F32);
        // }
        // else if self.peek(Token::TYPE_CHAR) {
        //     self.expect(Token::TYPE_CHAR);
        // }
        else if self.peek(Token::TYPE_BOOL) {
            self.expect(Token::TYPE_BOOL);
        }
        else {
            panic!("Did not expect '{:?}'!", self.curr());
        }
    }

    fn lit_check(&mut self) {
        if self.peek(Token::LIT_I32(0)) {
            self.parse_expression();
        }
        // else if self.peek(Token::LIT_F32(0.0)) {
        //     self.expect(Token::lit_f32());
        // }
        // else if self.peek(Token::LIT_CHAR(' ')) {
        //     self.expect(Token::lit_char());
        // }
        else if self.peek(Token::lit_string()) {
            self.expect(Token::lit_string());
        }
        else if self.peek(Token::lit_bool()) {
            self.parse_expression();
        }
        else {
            panic!("Did not expect '{:?}'!", self.curr());
        }
    }

    fn parse_next(&mut self) {
        if self.peek(Token::KW_LET) {
            self.parse_let();
        }
        else if self.peek(Token::KW_IF) {
            self.parse_if();
        }
        else if self.peek(Token::KW_RETURN) {
            self.parse_return();
        }
        else if self.peek(Token::KW_WHILE) {
            self.parse_while();
        }
        else if self.peek(Token::KW_PRINT) {
            self.parse_print();
        }
        else if self.peek(Token::id()) {
            self.parse_assignment();
        }
        else {
            panic!("Did not expect '{:?}'!", self.curr());
        }
    }

    fn parse_block_nest(&mut self) {
        self.indent_print("parse_block_nest()");
        self.indent_increment();
        {
            self.expect(Token::BRACKET_L);
            if self.peek(Token::BRACKET_L) {
                self.parse_block_list();
            }
            self.expect(Token::BRACKET_R);
        }
        self.indent_decrement();
    }

    fn parse_block_list(&mut self) {
        self.indent_print("parse_block_list()");
        self.indent_increment();
        {
            self.parse_block_nest();
            if self.peek(Token::BRACKET_L) {
                self.parse_block_list()
            }
        }
        self.indent_decrement();
    }

    fn parse_brace_nest(&mut self) {
        self.indent_print("parse_brace_nest()");
        self.indent_increment();
        {
            self.expect(Token::BRACE_L);
            if self.peek(Token::BRACE_L) {

                self.parse_brace_list();

            }
            while !self.peek(Token::BRACE_R) {
                self.parse_next();
            }
            self.expect(Token::BRACE_R);
        }
        self.indent_decrement();
    }

    fn parse_brace_list(&mut self) {
        self.indent_print("parse_brace_list()");
        self.indent_increment();

        {
            self.parse_brace_nest();

            if self.peek(Token::BRACE_L) {

                self.parse_brace_nest();

            }

        }
        self.indent_decrement();
    }

    fn parse_let(&mut self) {
        /*
            Let -> Let id (: type)* ('=' (id|lit))* ';'
        */
        self.indent_print("parse_let()");
        self.indent_increment();
        {
            self.expect(Token::KW_LET);
            self.expect(Token::id());
            if self.peek(Token::SEMICOLON) {
                self.expect(Token::SEMICOLON);
            }
            else {
                if self.accept(Token::COLON) {
                    self.type_check();
                    if self.peek(Token::SEMICOLON) {
                        self.expect(Token::SEMICOLON);
                    }
                }

                self.expect(Token::OP_ASSIGN);
                //TODO: NICE EXPRESSION
                if self.peek(Token::ID(String::new())) {
                    self.expect(Token::id());
                }
                else {
                    self.lit_check();
                }
                self.expect(Token::SEMICOLON);
            }
        }
        self.indent_decrement();
    }

    fn parse_if(&mut self) {
        /*
            If -> if List Body (else Body)*
            List -> ('(' (expr)* ')')? | (expr)*
            Body -> {(Let)* (If)* (Return)*}
        */

        self.indent_print("parse_if()");
        self.indent_increment();
        {
            self.expect(Token::KW_IF);
            if self.accept(Token::PAREN_L) {
                if self.peek(Token::ID(String::new())) {
                    self.parse_expression();
                }
                else {
                    self.lit_check();
                }
                self.expect(Token::PAREN_R);
            }
            else {
                //TODO: NICE EXPRESSION
                if self.peek(Token::ID(String::new())) {
                    self.parse_expression();
                }
                else {
                    self.lit_check();
                }
            }

            self.parse_brace_nest();
            if self.peek(Token::KW_ELSE) {
                self.indent_print("parse_else()");
                if self.accept(Token::KW_ELSE) {
                    self.parse_brace_nest();
                }
            }
        }
        self.indent_decrement();
    }

    fn parse_return(&mut self) {
        /*
            Return -> return (lit|id)* ';'
        */
        self.indent_print("parse_return()");
        self.indent_increment();
        {
            self.expect(Token::KW_RETURN);
            if !self.peek(Token::SEMICOLON) {
                if self.peek(Token::ID(String::new())) {
                    self.parse_expression();
                }
                else {
                    self.lit_check();
                }
            }
            self.expect(Token::SEMICOLON);
        }
        self.indent_decrement();
    }

    fn parse_while(&mut self) {
        /*
            While -> while expression+ { } ';'
        */
        self.indent_print("parse_while()");
        self.indent_increment();
        {
            self.expect(Token::KW_WHILE);
            if self.accept(Token::PAREN_L) {
                self.parse_expression();
                self.expect(Token::PAREN_R);
            }
            else {
                self.parse_expression();
            }
            self.parse_brace_nest();
        }
        self.indent_decrement();
    }

    fn parse_print(&mut self) {
        /*
            Print -> print (returning func-call | int | bool)+
         */
        self.indent_print("parse_print()");
        self.indent_increment();
        {
            self.expect(Token::KW_PRINT);
            self.parse_expression();
            self.expect(Token::SEMICOLON);
        }
        self.indent_decrement();
    }

    fn parse_expression(&mut self) -> Vec<Token> {
        /*
            Expressions ->
         */
        self.indent_print("parse_expression()");
        self.indent_increment();

        let mut expression = vec![];
        {
            let mut last;
            while self.peek(Token::lit_i32()) ||
                self.peek(Token::id()) ||
                self.peek(Token::lit_bool()) ||
                self.peek(Token::OP_ADD) ||
                self.peek(Token::OP_SUB) ||
                self.peek(Token::OP_MUL) ||
                self.peek(Token::OP_DIV) ||
                self.peek(Token::OP_AND_BIT) ||
                self.peek(Token::OP_OR_BIT) ||
                self.peek(Token::OP_NOT) ||
                self.peek(Token::OP_EQUAL) ||
                self.peek(Token::OP_LT) ||
                self.peek(Token::OP_GT) {

                last = self.curr().clone();
                self.advance();
                if self.peek(Token::PAREN_L) {
                    self.expect(Token::PAREN_L);

                    let mut tokens = vec![];

                    //TODO: BEGIN - Expression =
                    if self.peek(Token::lit_i32()) ||
                        self.peek(Token::id()) ||
                        self.peek(Token::lit_bool()) {

                        tokens = self.parse_expression();
                    }

                    last = Token::CALLS(tokens.clone());
                    expression.push(last.clone());

                    while self.accept(Token::COMMA) {
                        tokens = self.parse_expression();
                        last = Token::CALLS(tokens.clone());
                        expression.push(last.clone());
                    }

                    //TODO: END
                    self.expect(Token::PAREN_R);

                } else {
                    expression.push(last.clone());
                }
            }
            brad_pratt(expression.clone());
        }
        println!("{:<indent$}{:?}", "", expression.clone(), indent=self.indent);
        self.indent_decrement();
        return expression.clone();
    }

    fn parse_assignment(&mut self) {
        self.indent_print("parse_assignment()");
        self.indent_increment();
        {
            self.expect(Token::id());
            self.expect(Token::OP_ASSIGN);
            self.parse_expression();
            self.expect(Token::SEMICOLON);
        }
        self.indent_decrement();
    }
}


impl DescentParser { // utility functions for lexer

    fn curr(&mut self) -> Token {
        self.lexer.current()
    }

    fn advance(&mut self) {
        self.lexer.advance();
    }

    fn expect(&mut self, symbol: Token) {
        if self.curr() == symbol {
            self.advance();
            println!("{:<indent$}expect({symbol:?})", "", indent = self.indent);
        } else {
            panic!("Did not expect '{symbol:?}'!");
        }
    }

    fn accept(&mut self, symbol: Token) -> bool {
        if self.curr() == symbol {
            println!("{:<indent$}accept({symbol:?})", "", indent = self.indent);
            self.advance();
            true
        } else {
            false
        }
    }

    fn peek(&mut self, symbol: Token) -> bool {
        self.lexer.current() == symbol
    }

}


impl DescentParser { // utility functions for pretty print

    fn indent_print(&mut self, msg: &'static str) {
        println!("{:<indent$}{:}", "", msg, indent=self.indent);
    }

    fn indent_increment(&mut self) {
        self.indent += INDENT;
    }
    fn indent_decrement(&mut self) {
        self.indent -= INDENT;
    }

}
