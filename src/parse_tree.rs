#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_mut)]

use std::rc::Rc;
use crate::token::Token;
use crate::tree::ExprNode;
use crate::value::Value;

#[derive(Clone)]
pub struct ParseTree {
    token : Token,
    pub(crate) children : Vec<Box<ParseTree>>,
}

impl ParseTree {

    pub fn new(token : Token) -> ParseTree {
        ParseTree {
            token,
            children : vec![],
        }
    }

    pub fn push(&mut self, tree : ParseTree) {
        self.children.push(Box::<ParseTree>::new(tree));
    }

    pub fn node_string(&self) -> String {
        format!("{:?}", self.token)
    }

    fn print_recursively(&self, level : usize) {
        let shift = 2*level;
        print!("{:1$}", "", shift);
        println!("{}", self.node_string());
        for child in &self.children {
            child.as_ref().print_recursively(level+1);
        }
    }

    pub fn evaluate_recursively(&self) -> ExprNode {
        let mut expression = ExprNode::Val(Value::Nil);

        if self.children.is_empty() {
            if self.token.eq(&Token::id()) {
                let mut str = self.node_string();
                str = str[4..str.len()-2].parse().unwrap();
                //println!("{}", str);
                expression = ExprNode::Var(str);
            }
            else if self.token.eq(&Token::lit_i32()) {
                let mut str = self.node_string();
                str = str[8..str.len()-1].parse().unwrap();
                //println!("{}", str);
                expression = ExprNode::Val(Value::I32(str.parse::<i32>().unwrap()));
            }
            else if self.token.eq(&Token::lit_bool()) {
                let mut str = self.node_string();
                str = str[9..str.len()-1].parse().unwrap();
                //println!("{}", str);
                expression = ExprNode::Val(Value::Bool(str.parse::<bool>().unwrap()));
            }
            else if self.token.eq(&Token::calls()) {
                let mut str = self.node_string().clone();
                let mut strs = str.split("[");
                let mut str1 = strs.clone().nth(0).unwrap().clone();
                str1 = &str1[7..str1.len() - 3];
                //println!("{}", str1);

                let mut str2 = strs.clone().nth(1).unwrap().clone();
                let mut expressionsForCall = vec![];
                if str2.contains(",") {
                    let mut exprsStr = str2.split(",");
                    let mut exprStr;
                    for i in 0..exprsStr.clone().count()-1 {
                        exprStr = exprsStr.clone().nth(i).unwrap().clone();
                        exprStr = exprStr.clone().trim();
                        if exprStr.clone().starts_with("ID") {
                            exprStr = &exprStr[4..exprStr.len()-2];
                            //println!("{}", exprStr);
                            expressionsForCall.push(Rc::new(ExprNode::Var(exprStr.to_string())));
                        }
                        else if exprStr.clone().starts_with("LIT_I32") {
                            exprStr = &exprStr[8..exprStr.len()-1];
                            //println!("{}", exprStr);
                            expressionsForCall.push(Rc::new(ExprNode::Val(Value::I32(exprStr.parse::<i32>().unwrap()))));
                        }
                        else if exprStr.clone().starts_with("LIT_BOOL") {
                            exprStr = &exprStr[9..exprStr.len()-1];
                            //println!("{}", exprStr);
                            expressionsForCall.push(Rc::new(ExprNode::Val(Value::Bool(exprStr.parse::<bool>().unwrap()))));
                        }
                    }

                    let mut exprStr1 = exprsStr.clone().last().unwrap().clone();
                    exprStr1 = exprStr1.clone().trim();
                    if exprStr1.clone().starts_with("ID") {
                        exprStr1 = &exprStr1[4..exprStr1.len()-4];
                        //println!("{}", exprStr1);
                        expressionsForCall.push(Rc::new(ExprNode::Var(exprStr1.to_string())));
                    }
                    else if exprStr1.clone().starts_with("LIT_I32") {
                        exprStr1 = &exprStr1[8..exprStr1.len()-3];
                        //println!("{}", exprStr1);
                        expressionsForCall.push(Rc::new(ExprNode::Val(Value::I32(exprStr1.parse::<i32>().unwrap()))));
                    }
                    else if exprStr1.clone().starts_with("LIT_BOOL") {
                        exprStr1 = &exprStr1[9..exprStr1.len()-3];
                        //println!("{}", exprStr1);
                        expressionsForCall.push(Rc::new(ExprNode::Val(Value::Bool(exprStr1.parse::<bool>().unwrap()))));
                    }
                    //println!("{:?}", expressionsForCall);
                }
                else {
                    let mut exprStr1 = str2.clone();
                    exprStr1 = exprStr1.clone().trim();
                    if exprStr1.clone().starts_with("ID") {
                        exprStr1 = &exprStr1[4..exprStr1.len()-4];
                        //println!("{}", exprStr1);
                        expressionsForCall.push(Rc::new(ExprNode::Var(exprStr1.to_string())));
                    }
                    else if exprStr1.clone().starts_with("LIT_I32") {
                        exprStr1 = &exprStr1[8..exprStr1.len()-3];
                        //println!("{}", exprStr1);
                        expressionsForCall.push(Rc::new(ExprNode::Val(Value::I32(exprStr1.parse::<i32>().unwrap()))));
                    }
                    else if exprStr1.clone().starts_with("LIT_BOOL") {
                        exprStr1 = &exprStr1[9..exprStr1.len()-3];
                        //println!("{}", exprStr1);
                        expressionsForCall.push(Rc::new(ExprNode::Val(Value::Bool(exprStr1.parse::<bool>().unwrap()))));
                    }
                    //println!("{:?}", expressionsForCall);
                }
                expression = ExprNode::Call(str1.parse().unwrap(), expressionsForCall);
            }
        }

        if !self.children.is_empty() {
            if self.children.len() == 1 {
                if self.token.eq(&Token::OP_NOT) {
                    expression = ExprNode::Not(Rc::new(self.children.first().unwrap().evaluate_recursively()));
                }
            }
            else {
                //Create ExpressionNode for Unary Operator Based off of Parent (Token Name)
                if self.token.eq(&Token::OP_ADD) {
                    expression = ExprNode::Add(Rc::new(self.children.first().unwrap().evaluate_recursively()),
                                  Rc::new(self.children.last().unwrap().evaluate_recursively()));
                }
                else if self.token.eq(&Token::OP_SUB) {
                    expression = ExprNode::Sub(Rc::new(self.children.first().unwrap().evaluate_recursively()),
                                               Rc::new(self.children.last().unwrap().evaluate_recursively()));
                }
                else if self.token.eq(&Token::OP_MUL) {
                    expression = ExprNode::Mul(Rc::new(self.children.first().unwrap().evaluate_recursively()),
                                               Rc::new(self.children.last().unwrap().evaluate_recursively()));
                }
                else if self.token.eq(&Token::OP_DIV) {
                    expression = ExprNode::Div(Rc::new(self.children.first().unwrap().evaluate_recursively()),
                                               Rc::new(self.children.last().unwrap().evaluate_recursively()));
                }
                else if self.token.eq(&Token::OP_LT) {
                    expression = ExprNode::LT(Rc::new(self.children.first().unwrap().evaluate_recursively()),
                                               Rc::new(self.children.last().unwrap().evaluate_recursively()));
                }
                else if self.token.eq(&Token::OP_GT) {
                    expression = ExprNode::GT(Rc::new(self.children.first().unwrap().evaluate_recursively()),
                                              Rc::new(self.children.last().unwrap().evaluate_recursively()));
                }
                else if self.token.eq(&Token::OP_EQUAL) {
                    expression = ExprNode::EQUAL(Rc::new(self.children.first().unwrap().evaluate_recursively()),
                                              Rc::new(self.children.last().unwrap().evaluate_recursively()));
                }
                else if self.token.eq(&Token::OP_OR_BIT) {
                    expression = ExprNode::OR_BIT(Rc::new(self.children.first().unwrap().evaluate_recursively()),
                                              Rc::new(self.children.last().unwrap().evaluate_recursively()));
                }
                else if self.token.eq(&Token::OP_AND_BIT) {
                    expression = ExprNode::AND_BIT(Rc::new(self.children.first().unwrap().evaluate_recursively()),
                                                  Rc::new(self.children.last().unwrap().evaluate_recursively()));
                }
            }
        }
        expression.clone()
    }

    pub fn print(&mut self) {
        //self.print_recursively(0);
        self.evaluate_recursively();
    }
}
