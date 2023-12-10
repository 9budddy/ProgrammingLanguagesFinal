#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(dead_code)]

use std::env;
use std::rc::Rc;
use crate::machine::Machine;
use crate::parser::boolS;

pub fn runNeg1(_contents: String) {

    let argc: Vec<String> = env::args().collect();

    let prog = boolS(_contents);

    if argc.get(1).unwrap().chars().find(|chars| { chars == &'d' || chars == &'e' }).is_some() {
        let runtime = Machine::new(Rc::new(prog));
        runtime.run();
    }
}