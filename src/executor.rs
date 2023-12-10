#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::cell::RefCell;
use std::env;
use std::ops::Deref;
use std::rc::Rc;
use crate::evaluator::Evaluator;
use crate::frame::Frame;
use crate::tree::{BlockNode, FuncNode, ProgramNode, StmtNode};
use crate::value::Value;

pub struct Executor {
    program: Rc<ProgramNode>,
}

impl Executor {

    pub fn new(program: Rc<ProgramNode>) -> Executor {
        Executor { program }
    }

    pub fn execute(&self) {
        let argc: Vec<String> = env::args().collect();
        if argc.get(1).unwrap().chars().find(|chars| { chars == &'d' }).is_some() {
            println!("[info] Execute.");
        }
        self.execute_program();
    }

    fn execute_program(&self) {
        let argc: Vec<String> = env::args().collect();
        if argc.get(1).unwrap().chars().find(|chars| { chars == &'d' }).is_some() {
            println!("[info] Execute Program.");
        }


        // get program node symbol table
        let rc_symbols = self.program.symbols.clone();
        let symbols = rc_symbols.borrow();

        // find main function node
        let rc_main = if let Some(main) = symbols.map.get("main") {
            match &main.value {
                Value::Func(rc_main, _) => { rc_main.clone() }
                _ => { panic!("Symbol 'main' is not a function!"); }
            }
        } else {
            panic!("Cannot find 'main' symbol!");
        };

        // create global stack frame
        let mut global = Frame::new(None);
        global.init_symbols(symbols.deref());
        let rc_global = Rc::new(RefCell::new(global));

        // execute main function
        let arguments = vec![Value::I32(1)];
        Self::execute_function(rc_main, rc_global, arguments);
    }

    pub fn execute_function(
        rc_func: Rc<FuncNode>,
        frame: Rc<RefCell<Frame>>,
        arguments: Vec<Value>
    ) -> Value
    {
        let name = &rc_func.name;
        let argc: Vec<String> = env::args().collect();
        if argc.get(1).unwrap().chars().find(|chars| { chars == &'d' }).is_some() {
            println!("[debug] calling function '{name}'.");
        }


        // create local stack frame
        let mut locals = Frame::new(Some(frame));

        // initialize parameters
        let name = &rc_func.name;
        if rc_func.numParameters() > arguments.len() {
            panic!("Not enough arguments for function {name}!");
        }
        if rc_func.numParameters() < arguments.len() {
            panic!("To many arguments for function {name}!");
        }
        locals.init_parameters(&rc_func.parameters, arguments);

        // execute function block
        let rc_block = rc_func.block_node.clone();
        let rc_locals = Rc::new(RefCell::new(locals));
        let return_value = Self::execute_block(rc_block, rc_locals);

        return_value.1
    }

    fn execute_block(
        rc_block: Rc<BlockNode>,
        rc_locals: Rc<RefCell<Frame>>,
    ) -> (bool, Value) {
        // get block node symbol table
        let rc_symbols = rc_block.symbols.clone();
        let symbols = rc_symbols.borrow_mut();

        // initialize local frame
        rc_locals.borrow_mut().init_symbols(&symbols);

        let argc: Vec<String> = env::args().collect();
        if argc.get(1).unwrap().chars().find(|chars| { chars == &'d' }).is_some() {
            println!("[debug] Block Symbols:");
            rc_locals.borrow_mut().print();
        }


        // execute statements
        for statement in &rc_block.statements {
            let (done, value) = Self::execute_statement(
                statement.clone(),
                rc_locals.clone(),
            );
            if done {
                return (true, value);
            }
        }

        (false, Value::Nil)
    }

    fn execute_statement(
        rc_statement: Rc<StmtNode>,
        rc_locals: Rc<RefCell<Frame>>,
    ) -> (bool, Value)
    {
        match rc_statement.deref() {
            StmtNode::If(ifs) => {
                let argc: Vec<String> = env::args().collect();
                if argc.get(1).unwrap().chars().find(|chars| { chars == &'d' }).is_some() {
                    println!("[debug] executing if statement");
                }

                let value = Evaluator::evaluate(ifs.expr.clone(), rc_locals.clone());
                let mut values;
                if value == Value::Bool(true) {
                    values = Self::execute_block(ifs.then.clone(), rc_locals.clone());
                }
                else {
                    values = Self::execute_block(ifs.elses.clone(), rc_locals.clone());
                }
                (true, values.1)
            }
            StmtNode::While(whiles) => {
                let argc: Vec<String> = env::args().collect();
                if argc.get(1).unwrap().chars().find(|chars| { chars == &'d' }).is_some() {
                    println!("[debug] executing while statement");
                }

                let mut value = Evaluator::evaluate(whiles.expr.clone(), rc_locals.clone());
                let mut values;
                while value == Value::Bool(true) {
                    values = Self::execute_block(whiles.then.clone(), rc_locals.clone());
                    value = Evaluator::evaluate(whiles.expr.clone(), rc_locals.clone());
                }
                (false, Value::Nil)
            }
            StmtNode::Let(lets) => {
                let argc: Vec<String> = env::args().collect();
                if argc.get(1).unwrap().chars().find(|chars| { chars == &'d' }).is_some() {
                    println!("[debug] executing let statement");
                }

                let name = &lets.name;
                rc_locals.borrow_mut().assign(name, Value::Null);
                (false, Value::Nil)
            }
            StmtNode::Assign(assign) => {
                let argc: Vec<String> = env::args().collect();
                if argc.get(1).unwrap().chars().find(|chars| { chars == &'d' }).is_some() {
                    println!("[debug] executing assign statement");
                }

                let name = &assign.name;
                let value = Evaluator::evaluate(assign.expr.clone(), rc_locals.clone());
                if rc_locals.borrow_mut().lookup(name) != Value::Nil  {
                    rc_locals.borrow_mut().assign(name, value.clone());
                    if argc.get(1).unwrap().chars().find(|chars| { chars == &'d' }).is_some() {
                        println!("[debug] assigning value {:?} to name {}", value, name);
                    }

                }
                else if rc_locals.borrow_mut().lookup_global(name) != Value::Nil {
                    rc_locals.borrow_mut().assign_global(name, value.clone());
                    if argc.get(1).unwrap().chars().find(|chars| { chars == &'d' }).is_some() {
                        println!("[debug] assigning global_value {:?} to global_name {}", value, name);
                    }

                }
                else {
                    panic!("Missing let declaration for variable {:?}", name);
                }

                (false, Value::Nil)
            }
            StmtNode::Return(ret) => {
                let argc: Vec<String> = env::args().collect();
                if argc.get(1).unwrap().chars().find(|chars| { chars == &'d' }).is_some() {
                    println!("[debug] executing return statement");
                }

                let value = Evaluator::evaluate(ret.expr.clone(), rc_locals.clone());
                if argc.get(1).unwrap().chars().find(|chars| { chars == &'d' }).is_some() {
                    println!("[debug] returning value {:?}", value);
                }
                (true, value)
            }
            StmtNode::Print(print) => {
                let argc: Vec<String> = env::args().collect();
                if argc.get(1).unwrap().chars().find(|chars| { chars == &'d' }).is_some() {
                    println!("[debug] executing print statement");
                }

                let value = Evaluator::evaluate(print.expr.clone(), rc_locals.clone());
                value.print();
                (false, Value::Nil)
            }
        }

    }

}