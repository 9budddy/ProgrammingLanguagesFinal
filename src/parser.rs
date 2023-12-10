#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_mut)]

use std::env;
use std::rc::Rc;
use crate::lexer_mockup::Lexer;
use crate::lexer::MyLexer;
use crate::pratt_parsing::brad_pratt;
use crate::token::Token;
use crate::tree::{AssignNode, BlockNode, ExprNode, FuncNode, IfNode, LetNode, Parameter, PrintNode, ProgramNode, ReturnNode, StmtNode, WhileNode};
use crate::value::Value;

const INDENT : usize = 2;


pub fn boolS(_contents: String) -> ProgramNode {
    let argc: Vec<String> = env::args().collect();

    let mut lexer;
    lexer = MyLexer::set_input(_contents);

    let mut tokens = lexer.get_tokens();
    if argc.get(1).unwrap().chars().find(|chars| { chars == &'t' }).is_some() {
        println!("{:?}", tokens);
    }
    let mut prog = ProgramNode::new();
    if argc.get(1).unwrap().chars().find(|chars| { chars == &'d' || chars == &'e' || chars == &'p'}).is_some() {
        let lexer = Lexer::new(tokens);
        let mut parser = DescentParser::new(lexer);

        prog = parser.analyze();
    }
    prog

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

    pub fn analyze(&mut self) -> ProgramNode {
        self.indent = 0;
        let mut prog = ProgramNode::new();
        while !self.peek(Token::EOI) {
            if self.peek(Token::KW_FUNC) {
                let mut x = self.parse_func();
                prog.func_nodes.push(Rc::new(x));

            }
            else if self.peek(Token::KW_LET) {
                let mut x = self.parse_let();
                prog.let_nodes.push(Rc::new(x));
            }
            //else if self.peek(Token::id()) {
            //    self.parse_assignment();
            //}
        }
        self.expect(Token::EOI);
        prog
    }

    fn parse_func(&mut self) -> FuncNode {
        /*
            Func -> func id List ("->" id)* Body
            List -> '(' (Param(, Param)*)? ')'
            Param -> id ':' id
            Body -> {(Let)* (If)* (Return)*}
        */
        let argc: Vec<String> = env::args().collect();
        if argc.get(1).unwrap().chars().find(|chars| { chars == &'p' }).is_some() {
            self.indent_print("parse_func()");
        }

        self.indent_increment();
        let mut str;
        let mut parameters = vec![];
        let mut block = BlockNode::new();
        {
            self.expect(Token::KW_FUNC);
            str = format!("{:?}", self.curr());
            str = str[4..str.len()-2].parse().unwrap();
            self.expect(Token::id());
            parameters = self.parse_parameter_list();
            //if self.peek(Token::ARROW_R) {
            //    self.expect(Token::ARROW_R);
            //    self.type_check();
            //}
            block = self.parse_brace_nest();
        }
        self.indent_decrement();
        FuncNode::new(str, parameters, block)
    }

    fn parse_parameter_list(&mut self) -> Vec<Parameter> {
        let mut parameters : Vec<Parameter> = vec![];
        let argc: Vec<String> = env::args().collect();
        if argc.get(1).unwrap().chars().find(|chars| { chars == &'p' }).is_some() {
            self.indent_print("parse_parameter_list()");
        }

        self.indent_increment();
        {
            self.expect(Token::PAREN_L);
            if self.accept(Token::PAREN_R) {
                return parameters;
            }
            parameters.push(Parameter::new(self.parse_parameter()));
            while self.accept(Token::COMMA) {
                parameters.push(Parameter::new(self.parse_parameter()));
            }
            self.expect(Token::PAREN_R);
        }
        self.indent_decrement();
        parameters
    }

    fn parse_parameter(&mut self) -> String {
        let argc: Vec<String> = env::args().collect();
        if argc.get(1).unwrap().chars().find(|chars| { chars == &'p' }).is_some() {
            self.indent_print("parse_parameter()");
        }

        self.indent_increment();
        let mut str;
        {
            str = format!("{:?}", self.curr());
            str = str[4..str.len()-2].parse().unwrap();
            self.expect(Token::id());
            //self.expect(Token::COLON);
            //self.type_check();
        }
        self.indent_decrement();
        str
    }

    fn type_check(&mut self) {
        // if self.peek(Token::TYPE_I32) {
        //     self.expect(Token::TYPE_I32);
        // }
        // else if self.peek(Token::TYPE_F32) {
        //     self.expect(Token::TYPE_F32);
        // }
        // else if self.peek(Token::TYPE_CHAR) {
        //     self.expect(Token::TYPE_CHAR);
        // }
        // else if self.peek(Token::TYPE_BOOL) {
        //     self.expect(Token::TYPE_BOOL);
        // }
        // else {
        //     panic!("Did not expect '{:?}'!", self.curr());
        // }
    }

    fn lit_check(&mut self) -> ExprNode {
        let mut expr = ExprNode::Val(Value::Nil);
        if self.peek(Token::LIT_I32(0)) {
            let mut token = self.parse_expression();
            let mut tree = brad_pratt(token.clone());
            expr = tree.evaluate_recursively();
        }
        // else if self.peek(Token::LIT_F32(0.0)) {
        //     self.expect(Token::lit_f32());
        // }
        // else if self.peek(Token::LIT_CHAR(' ')) {
        //     self.expect(Token::lit_char());
        // }
        // else if self.peek(Token::lit_string()) {
        //     self.expect(Token::lit_string());
        // }
        else if self.peek(Token::lit_bool()) {
            let mut token = self.parse_expression();
            let mut tree = brad_pratt(token.clone());
            expr = tree.evaluate_recursively();
        }
        else {
            panic!("Did not expect '{:?}'!", self.curr());
        }
        expr.clone()
    }

    fn parse_next(&mut self, mut block: BlockNode) -> BlockNode {
        if self.peek(Token::KW_LET) {
            block.statements.push(Rc::new(StmtNode::Let(self.parse_let())));
        }
        else if self.peek(Token::KW_IF) {
            block.statements.push(Rc::new(self.parse_if()));
        }
        else if self.peek(Token::KW_RETURN) {
            block.statements.push(Rc::new(self.parse_return()));
        }
        else if self.peek(Token::KW_WHILE) {
            block.statements.push(Rc::new(self.parse_while()));
        }
        else if self.peek(Token::KW_PRINT) {
            block.statements.push(Rc::new(self.parse_print()));
        }
        else if self.peek(Token::id()) {
            block.statements.push(Rc::new(self.parse_assignment()));
        }
        else {
            panic!("Did not expect '{:?}'!", self.curr());
        }

        block.clone()
    }

    fn parse_block_nest(&mut self) {
        let argc: Vec<String> = env::args().collect();
        if argc.get(1).unwrap().chars().find(|chars| { chars == &'p' }).is_some() {
            self.indent_print("parse_block_nest()");
        }

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
        let argc: Vec<String> = env::args().collect();
        if argc.get(1).unwrap().chars().find(|chars| { chars == &'p' }).is_some() {
            self.indent_print("parse_block_list()");
        }

        self.indent_increment();
        {
            self.parse_block_nest();
            if self.peek(Token::BRACKET_L) {
                self.parse_block_list()
            }
        }
        self.indent_decrement();
    }

    fn parse_brace_nest(&mut self) -> BlockNode {
        let argc: Vec<String> = env::args().collect();
        if argc.get(1).unwrap().chars().find(|chars| { chars == &'p' }).is_some() {
            self.indent_print("parse_brace_nest()");
        }

        self.indent_increment();
        let mut block = BlockNode::new();
        {
            self.expect(Token::BRACE_L);
            if self.peek(Token::BRACE_L) {

                self.parse_brace_list();

            }
            while !self.peek(Token::BRACE_R) {
                block = self.parse_next(block.clone());
            }
            self.expect(Token::BRACE_R);
        }
        self.indent_decrement();
        block.clone()
    }

    fn parse_brace_list(&mut self) {
        let argc: Vec<String> = env::args().collect();
        if argc.get(1).unwrap().chars().find(|chars| { chars == &'p' }).is_some() {
            self.indent_print("parse_brace_list()");
        }

        self.indent_increment();

        {
            self.parse_brace_nest();

            if self.peek(Token::BRACE_L) {

                self.parse_brace_nest();

            }

        }
        self.indent_decrement();
    }

    fn parse_let(&mut self) -> LetNode {
        /*
            Let -> Let id (: type)* ('=' (id|lit))* ';'
        */
        let argc: Vec<String> = env::args().collect();
        if argc.get(1).unwrap().chars().find(|chars| { chars == &'p' }).is_some() {
            self.indent_print("parse_let()");
        }

        self.indent_increment();
        let mut str;
        {
            self.expect(Token::KW_LET);
            str = format!("{:?}", self.curr());
            str = str[4..str.len()-2].parse().unwrap();
            self.expect(Token::id());
            if self.peek(Token::SEMICOLON) {
                self.expect(Token::SEMICOLON);
            }
            // else {
            //     if self.accept(Token::COLON) {
            //         self.type_check();
            //         if self.peek(Token::SEMICOLON) {
            //             self.expect(Token::SEMICOLON);
            //         }
            //     }
            //
            //     self.expect(Token::OP_ASSIGN);
            //
            //     if self.peek(Token::ID(String::new())) {
            //         self.expect(Token::id());
            //     }
            //     else {
            //         self.lit_check();
            //     }
            //     self.expect(Token::SEMICOLON);
            // }
        }
        self.indent_decrement();
        LetNode::new(str)
    }

    fn parse_if(&mut self) -> StmtNode {
        /*
            If -> if List Body (else Body)*
            List -> ('(' (expr)* ')')? | (expr)*
            Body -> {(Let)* (If)* (Return)*}
        */
        let mut expr;
        let mut thenblock = BlockNode::new();
        let mut elseblock = BlockNode::new();
        let argc: Vec<String> = env::args().collect();
        if argc.get(1).unwrap().chars().find(|chars| { chars == &'p' }).is_some() {
            self.indent_print("parse_if()");
        }

        self.indent_increment();
        {
            self.expect(Token::KW_IF);
            if self.accept(Token::PAREN_L) {
                if self.peek(Token::ID(String::new())) {
                    let mut token = self.parse_expression();
                    let mut tree = brad_pratt(token.clone());
                    expr = tree.evaluate_recursively();
                }
                else {
                    expr = self.lit_check();
                }
                self.expect(Token::PAREN_R);
            }
            else {
                if self.peek(Token::ID(String::new())) {
                    let mut token = self.parse_expression();
                    let mut tree = brad_pratt(token.clone());
                    expr = tree.evaluate_recursively();
                }
                else {
                    expr = self.lit_check();
                }
            }

            thenblock = self.parse_brace_nest();
            if self.peek(Token::KW_ELSE) {
                let argc: Vec<String> = env::args().collect();
                if argc.get(1).unwrap().chars().find(|chars| { chars == &'p' }).is_some() {
                    self.indent_print("parse_else()");
                }

                if self.accept(Token::KW_ELSE) {
                    elseblock = self.parse_brace_nest();
                }
            }
        }
        self.indent_decrement();
        StmtNode::If(IfNode::new(expr, thenblock, elseblock))
    }

    fn parse_return(&mut self) -> StmtNode {
        /*
            Return -> return (lit|id)* ';'
        */

        let argc: Vec<String> = env::args().collect();
        if argc.get(1).unwrap().chars().find(|chars| { chars == &'p' }).is_some() {
            self.indent_print("parse_return()");
        }

        self.indent_increment();
        let mut expr = ExprNode::Val(Value::Nil);
        {
            self.expect(Token::KW_RETURN);
            if !self.peek(Token::SEMICOLON) {
                if self.peek(Token::ID(String::new())) {
                    let mut token = self.parse_expression();
                    let mut tree = brad_pratt(token.clone());
                    expr = tree.evaluate_recursively();
                }
                else {
                    expr = self.lit_check();
                }
            }
            self.expect(Token::SEMICOLON);
        }
        self.indent_decrement();
        StmtNode::Return(ReturnNode::new(expr))
    }

    fn parse_while(&mut self) -> StmtNode {
        /*
            While -> while expression+ { } ';'
        */
        let argc: Vec<String> = env::args().collect();
        if argc.get(1).unwrap().chars().find(|chars| { chars == &'p' }).is_some() {
            self.indent_print("parse_while()");
        }

        self.indent_increment();
        let mut expr = ExprNode::Val(Value::Nil);
        let mut thenblock = BlockNode::new();
        {
            self.expect(Token::KW_WHILE);
            if self.accept(Token::PAREN_L) {

                let mut token = self.parse_expression();
                let mut tree = brad_pratt(token.clone());
                expr = tree.evaluate_recursively();
                self.expect(Token::PAREN_R);
            }
            else {
                let mut token = self.parse_expression();
                let mut tree = brad_pratt(token.clone());
                expr = tree.evaluate_recursively();
            }
            thenblock = self.parse_brace_nest();
        }
        self.indent_decrement();
        StmtNode::While(WhileNode::new(expr, thenblock))
    }

    fn parse_print(&mut self) -> StmtNode {
        /*
            Print -> print (returning func-call | int | bool)+
         */
        let argc: Vec<String> = env::args().collect();
        if argc.get(1).unwrap().chars().find(|chars| { chars == &'p' }).is_some() {
            self.indent_print("parse_print()");
        }

        self.indent_increment();
        let mut expr = ExprNode::Val(Value::Nil);
        {
            self.expect(Token::KW_PRINT);
            let mut token = self.parse_expression();
            let mut tree = brad_pratt(token.clone());
            expr = tree.evaluate_recursively();
            self.expect(Token::SEMICOLON);
        }
        self.indent_decrement();
        StmtNode::Print(PrintNode::new(expr))
    }

    fn parse_expression(&mut self) -> Vec<Token> {
        /*
            Expressions ->
         */
        let argc: Vec<String> = env::args().collect();
        if argc.get(1).unwrap().chars().find(|chars| { chars == &'p' }).is_some() {
            self.indent_print("parse_expression()");
        }

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

                    let mut tokenVecs = vec![];
                    let mut tokens = vec![];
                    //BEGIN - Expression =
                    if self.peek(Token::lit_i32()) ||
                        self.peek(Token::id()) ||
                        self.peek(Token::lit_bool()) {

                        tokenVecs.push(self.parse_expression());
                    }

                    while self.accept(Token::COMMA) {
                        tokenVecs.push(self.parse_expression());
                    }

                    for tokenVec in tokenVecs {
                        for token in tokenVec {
                            tokens.push(token);
                        }
                    }
                    let mut str = format!("{:?}", last);
                    str = str[4..str.len()-2].parse().unwrap();
                    last = Token::CALLS(str,tokens.clone());
                    expression.push(last.clone());
                    //END
                    self.expect(Token::PAREN_R);

                } else {
                    expression.push(last.clone());
                }
            }
        }
        let argc: Vec<String> = env::args().collect();
        if argc.get(1).unwrap().chars().find(|chars| { chars == &'p' }).is_some() {
            println!("{:<indent$}{:?}", "", expression.clone(), indent=self.indent);
        }

        self.indent_decrement();
        return expression.clone();
    }

    fn parse_assignment(&mut self) -> StmtNode {
        let argc: Vec<String> = env::args().collect();
        if argc.get(1).unwrap().chars().find(|chars| { chars == &'p' }).is_some() {
            self.indent_print("parse_assignment()");
        }

        self.indent_increment();
        let mut str;
        let mut expr = ExprNode::Val(Value::Nil);
        {
            str = format!("{:?}", self.curr());
            str = str[4..str.len()-2].parse().unwrap();
            self.expect(Token::id());
            self.expect(Token::OP_ASSIGN);
            let mut token = self.parse_expression();
            let mut tree = brad_pratt(token.clone());
            expr = tree.evaluate_recursively();
            self.expect(Token::SEMICOLON);
        }
        self.indent_decrement();
        StmtNode::Assign(AssignNode::new(str, expr))
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
            let argc: Vec<String> = env::args().collect();
            if argc.get(1).unwrap().chars().find(|chars| { chars == &'p' }).is_some() {
                println!("{:<indent$}expect({symbol:?})", "", indent = self.indent);
            }

        } else {
            panic!("Did not expect '{symbol:?}'!");
        }
    }

    fn accept(&mut self, symbol: Token) -> bool {
        if self.curr() == symbol {
            let argc: Vec<String> = env::args().collect();
            if argc.get(1).unwrap().chars().find(|chars| { chars == &'p' }).is_some() {
                println!("{:<indent$}accept({symbol:?})", "", indent = self.indent);
            }

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
