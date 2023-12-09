#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(dead_code)]

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::executor::Executor;
use crate::frame::Frame;
use crate::tree::ExprNode;
use crate::value::Value;

pub struct Evaluator {

}

impl Evaluator {

    pub fn evaluate(expr: Rc<ExprNode>, rc_frame: Rc<RefCell<Frame>>) -> Value {
        match expr.deref() {
            ExprNode::Var(name) => {
                rc_frame.borrow().lookup(name)
            }
            ExprNode::Val(value) => {
                value.clone()
            }
            ExprNode::Not(expr) =>{
                let value_a = Self::evaluate(expr.clone(), rc_frame.clone());
                Self::not(value_a)
            }
            ExprNode::EQUAL(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::equal(value_a, value_b)
            }
            ExprNode::AND_BIT(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::or_bit(value_a, value_b)
            }
            ExprNode::OR_BIT(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::or_bit(value_a, value_b)
            }
            ExprNode::GT(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::gt(value_a, value_b)
            }
            ExprNode::LT(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::lt(value_a, value_b)
            }
            ExprNode::Add(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::add(value_a, value_b)
            }
            ExprNode::Sub(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::sub(value_a, value_b)
            }
            ExprNode::Mul(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::mul(value_a, value_b)
            }
            ExprNode::Div(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::div(value_a, value_b)
            }
            ExprNode::Call(name, rc_exprs) => {
                println!("[debug] evaluating call '{name}'");
                match rc_frame.borrow().lookup_global(name) {
                    Value::Func(rc_func, argc) => {
                        assert_eq!(argc,rc_exprs.len());
                        let mut arguments = vec![];
                        for rc_expr in rc_exprs {
                            let arg = Self::evaluate(rc_expr.clone(), rc_frame.clone());
                            arguments.push(arg);
                        }
                        Executor::execute_function(rc_func, rc_frame.clone(), arguments)
                    }
                    _ => {
                        println!("[warn] function '{name}' not found");
                        Value::Nil
                    }
                }
            }
        }
    }
    fn not(value_a: Value) -> Value {
        match value_a {
            Value::Nil => { Value::Nil }
            Value::Bool(a) => { Value::Bool(!a) }
            Value::I32(_) => { Value::Nil }
            Value::F32(_) => { todo!() }
            Value::Chars(_) => { todo!() }
            Value::Func(_, _) => { todo!() }
            Value::Null => { Value::Null }
        }
    }
    fn equal(value_a: Value, value_b: Value) -> Value {
        match value_a {
            Value::Nil => { Value::Nil }
            Value::Bool(a) => {
                match value_b {
                Value::Nil => { Value::Nil }
                Value::Bool(b) => { Value::Bool(a == b) }
                Value::I32(_) => { Value::Nil }
                Value::Null => { Value::Null }
                _ => { Value::Nil }
                }
            }
            Value::I32(a) => {
                match value_b {
                    Value::Nil => { Value::Nil }
                    Value::Bool(_) => { Value::Nil }
                    Value::I32(b) => { Value::Bool(a == b) }
                    Value::Null => { Value::Null }
                    _ => { Value::Nil }
                }
            }
            Value::F32(_) => { todo!() }
            Value::Chars(_) => { todo!() }
            Value::Func(_, _) => { todo!() }
            Value::Null => { Value::Null }
        }
    }
    fn and_bit(value_a: Value, value_b: Value) -> Value {
        match value_a {
            Value::Nil => { Value::Nil }
            Value::Bool(a) => {
                match value_b {
                    Value::Nil => { Value::Nil }
                    Value::Bool(b) => { Value::Bool(a & b) }
                    Value::I32(_) => { Value::Nil }
                    Value::Null => { Value::Null }
                    _ => { Value::Nil }
                }
            }
            Value::I32(_) => { Value::Nil }
            Value::F32(_) => { todo!() }
            Value::Chars(_) => { todo!() }
            Value::Func(_, _) => { todo!() }
            Value::Null => { Value::Null }
        }
    }
    fn or_bit(value_a: Value, value_b: Value) -> Value {
        match value_a {
            Value::Nil => { Value::Nil }
            Value::Bool(a) => {
                match value_b {
                    Value::Nil => { Value::Nil }
                    Value::Bool(b) => { Value::Bool(a | b) }
                    Value::I32(_) => { Value::Nil }
                    Value::Null => { Value::Null }
                    _ => { Value::Nil }
                }
            }
            Value::I32(_) => { Value::Nil }
            Value::F32(_) => { todo!() }
            Value::Chars(_) => { todo!() }
            Value::Func(_, _) => { todo!() }
            Value::Null => { Value::Null }
        }
    }
    fn gt(value_a: Value, value_b: Value) -> Value {
        match value_a {
            Value::Nil => { Value::Nil }
            Value::Bool(_) => { Value::Nil }

            Value::I32(a) => {
                match value_b {
                    Value::Nil => { Value::Nil }
                    Value::Bool(_) => { Value::Nil }
                    Value::I32(b) => { Value::Bool(a > b) }
                    Value::Null => { Value::Null }
                    _ => { Value::Nil }
                }
            }
            Value::F32(_) => { todo!() }
            Value::Chars(_) => { todo!() }
            Value::Func(_, _) => { todo!() }
            Value::Null => { Value::Null }
        }
    }
    fn lt(value_a: Value, value_b: Value) -> Value {
        match value_a {
            Value::Nil => { Value::Nil }
            Value::Bool(_) => { Value::Nil }

            Value::I32(a) => {
                match value_b {
                    Value::Nil => { Value::Nil }
                    Value::Bool(_) => { Value::Nil }
                    Value::I32(b) => { Value::Bool(a < b) }
                    Value::Null => { Value::Null }
                    _ => { Value::Nil }
                }
            }
            Value::F32(_) => { todo!() }
            Value::Chars(_) => { todo!() }
            Value::Func(_, _) => { todo!() }
            Value::Null => { Value::Null }
        }
    }
    fn add(value_a: Value, value_b: Value) -> Value {
        match value_a {
            Value::Null => { Value::Null }
            Value::Nil => { Value::Nil }
            Value::Bool(a) => {
                match value_b {
                    Value::Nil => { Value::Nil }
                    Value::Bool(b) => { Value::I32(if a {1} else {0} + if b {1} else {0}) }
                    Value::I32(b) => { Value::I32(if a {1} else {0} + b) }
                    Value::Null => { Value::Null }
                    _ => { Value::Nil }
                }
            }
            Value::I32(a) => {
                match value_b {
                    Value::Null => { Value::Null }
                    Value::Nil => { Value::Nil }
                    Value::Bool(b) => { Value::I32(a + if b {1} else {0}) }
                    Value::I32(b) => { Value::I32(a + b) }
                    _ => { Value::Nil }
                }
            }
            Value::F32(_) => { todo!() }
            Value::Chars(_) => { todo!() }
            Value::Func(_, _) => { todo!() }
        }
    }
    fn sub(value_a: Value, value_b: Value) -> Value {
        match value_a {
            Value::Null => { Value::Null }
            Value::Nil => { Value::Nil }
            Value::Bool(a) => {
                match value_b {
                    Value::Nil => { Value::Nil }
                    Value::Bool(b) => { Value::I32(if a {1} else {0} - if b {1} else {0}) }
                    Value::I32(b) => { Value::I32(if a {1} else {0} - b) }
                    Value::Null => { Value::Null }
                    _ => { Value::Nil }
                }
            }
            Value::I32(a) => {
                match value_b {
                    Value::Null => { Value::Null }
                    Value::Nil => { Value::Nil }
                    Value::Bool(b) => { Value::I32(a - if b {1} else {0}) }
                    Value::I32(b) => { Value::I32(a - b) }
                    _ => { Value::Nil }
                }
            }
            Value::F32(_) => { todo!() }
            Value::Chars(_) => { todo!() }
            Value::Func(_, _) => { todo!() }
        }
    }
    fn mul(value_a: Value, value_b: Value) -> Value {
        match value_a {
            Value::Null => { Value::Null }
            Value::Nil => { Value::Nil }
            Value::Bool(a) => {
                match value_b {
                    Value::Nil => { Value::Nil }
                    Value::Bool(b) => { Value::I32(if a {1} else {0} * if b {1} else {0}) }
                    Value::I32(b) => { Value::I32(if a {1} else {0} * b) }
                    Value::Null => { Value::Null }
                    _ => { Value::Nil }
                }
            }
            Value::I32(a) => {
                match value_b {
                    Value::Null => { Value::Null }
                    Value::Nil => { Value::Nil }
                    Value::Bool(b) => { Value::I32(a * if b {1} else {0}) }
                    Value::I32(b) => { Value::I32(a * b) }
                    _ => { Value::Nil }
                }
            }
            Value::F32(_) => { todo!() }
            Value::Chars(_) => { todo!() }
            Value::Func(_, _) => { todo!() }
        }
    }
    fn div(value_a: Value, value_b: Value) -> Value {
        match value_a {
            Value::Null => { Value::Null }
            Value::Nil => { Value::Nil }
            Value::Bool(a) => {
                match value_b {
                    Value::Nil => { Value::Nil }
                    Value::Bool(b) => { Value::I32(if a {1} else {0} / if b {1} else {0}) }
                    Value::I32(b) => { Value::I32(if a {1} else {0} / b) }
                    Value::Null => { Value::Null }
                    _ => { Value::Nil }
                }
            }
            Value::I32(a) => {
                match value_b {
                    Value::Null => { Value::Null }
                    Value::Nil => { Value::Nil }
                    Value::Bool(b) => { Value::I32(a / if b {1} else {0}) }
                    Value::I32(b) => { Value::I32(a / b) }
                    _ => { Value::Nil }
                }
            }
            Value::F32(_) => { todo!() }
            Value::Chars(_) => { todo!() }
            Value::Func(_, _) => { todo!() }
        }
    }
}