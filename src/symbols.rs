#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub value: Value,
    pub signature: usize,
}

impl Symbol {
    pub fn new(name: String, value: Value, signature: usize) -> Symbol {
        Symbol { name, value, signature }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbols {
    pub parent: Option<Rc<RefCell<Symbols>>>,
    pub map: HashMap<String, Symbol>,
}

impl Symbols {

    pub fn new(parent: Option<Rc<RefCell<Symbols>>>) -> Symbols {
        Symbols {
            parent: parent.clone(),
            map: HashMap::new()
        }
    }

}