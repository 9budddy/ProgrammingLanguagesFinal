#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(dead_code)]

use std::rc::Rc;
use crate::machine::Machine;
use crate::parser::boolS;

pub fn runNeg1(argc: Vec<String>) {
    //Lexer
    //Parser
    //Semantical Analysis

    let prog = boolS(argc.clone());

    if argc.contains(&"e".to_string()) {
        let runtime = Machine::new(Rc::new(prog));
        runtime.run();
    }

}