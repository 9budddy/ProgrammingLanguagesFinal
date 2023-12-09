#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(dead_code)]

use std::rc::Rc;
use crate::tree::{FuncNode};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Nil,
    Bool(bool),
    I32(i32),
    F32(f32),
    Chars(String),
    Func(Rc<FuncNode>, usize),
}

impl Value {
    pub fn print(&self) {
        println!("{self:?}");
    }
}