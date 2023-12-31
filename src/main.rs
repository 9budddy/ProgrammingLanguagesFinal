#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(dead_code)]

use std::fs;
use crate::somethinggood::runNeg1;


mod tree;
mod executor;
mod machine;
mod analyzer;
mod symbols;
mod frame;
mod value;
mod evaluator;
mod somethinggood;
mod token;
mod lexer_mockup;
mod parser;
mod lexer;
mod pratt_parsing;
mod parse_tree;


/*fn grow_ast_program0() -> Rc<ProgramNode> {

    let mut program = ProgramNode::new();

    //GLOBALS
    let let_global = LetNode::new("global".to_string());
    program.let_nodes.push(Rc::new(let_global));




    // FACTORIAL_RECURSION function
    let mut parameters_factorial_recursion = vec![];
    parameters_factorial_recursion.push(Parameter::new("n".to_string()));
    parameters_factorial_recursion.push(Parameter::new("q".to_string()));
    parameters_factorial_recursion.push(Parameter::new("z".to_string()));

    let mut block_factorial_recursion = BlockNode::new();


    //--------HEY this de IF
    let express_if1 = ExprNode::LT(
        Rc::new(ExprNode::Var("n".to_string())),
        Rc::new(ExprNode::Val(Value::I32(2))),
    );
    let mut block_if1_then = BlockNode::new();
    let stmt_if1_then1 = StmtNode::Return(
        ReturnNode::new(ExprNode::Val(
            Value::I32(1)
        ))
    );
    block_if1_then.statements.push(Rc::new(stmt_if1_then1));

    let mut block_if1_elses = BlockNode ::new();
    let stmt_if1_elses1 = StmtNode::Return(
        ReturnNode::new(ExprNode::Var(
            "n".to_string()
        ))
    );
    block_if1_elses.statements.push(Rc::new(stmt_if1_elses1));


    let ifFactRec1 = StmtNode::If(
        IfNode::new(express_if1, block_if1_then, block_if1_elses)
    );
    block_factorial_recursion.statements.push(Rc::new(ifFactRec1));

    let func_FacRec = FuncNode::new(
        "factorial_recursion".to_string(),
        parameters_factorial_recursion,
        block_factorial_recursion);

    program.func_nodes.push(Rc::new(func_FacRec));



    //FACTORIAL_LOOP
    let mut parameters_factorial_loop = vec![];
    parameters_factorial_loop.push(Parameter::new("n".to_string()));

    let mut block_factorial_loop = BlockNode::new();
    let stmtFacLoop1 = StmtNode::Let(LetNode::new("p".to_string()));
    let stmtFacLoop2 = StmtNode::Assign(AssignNode::new("p".to_string(), ExprNode::Var("n".to_string())));

    let express_FacLoop = ExprNode::GT(
        Rc::new(ExprNode::Var("n".to_string())),
        Rc::new(ExprNode::Val(Value::I32(0))),
    );
    let mut block_FacLoop_then = BlockNode::new();

    let stmt_FacLoop_then1 = StmtNode::Assign(
        AssignNode::new("n".to_string(), ExprNode::Sub(
            Rc::new(ExprNode::Var("n".to_string())),
            Rc::new(ExprNode::Val(Value::I32(1))),
    )));
    let stmt_FacLoop_then2 = StmtNode::Assign(
        AssignNode::new("p".to_string(), ExprNode::Mul(
            Rc::new(ExprNode::Var("p".to_string())),
            Rc::new(ExprNode::Var("n".to_string())),
        ))
    );
    block_FacLoop_then.statements.push(Rc::new(stmt_FacLoop_then1));
    block_FacLoop_then.statements.push(Rc::new(stmt_FacLoop_then2));


    let stmtFacLoop3 = StmtNode::While(WhileNode::new(
        express_FacLoop,
        block_FacLoop_then,
    ));

    let stmtFacLoop5 = StmtNode::Return(ReturnNode::new(ExprNode::Var("p".to_string())));

    block_factorial_loop.statements.push(Rc::new(stmtFacLoop1));
    block_factorial_loop.statements.push(Rc::new(stmtFacLoop2));
    block_factorial_loop.statements.push(Rc::new(stmtFacLoop3));
    block_factorial_loop.statements.push(Rc::new(stmtFacLoop5));

    let func_FacLoop = FuncNode::new(
        "factorial_loop".to_string(),
        parameters_factorial_loop,
        block_factorial_loop);

    program.func_nodes.push(Rc::new(func_FacLoop));



    //MAIN PROGRAM
    let mut parameters_main : Vec<Parameter> = vec![];
    parameters_main.push(Parameter::new("argc".to_string()));

    let mut block_main = BlockNode::new();
    let stmtMain1 = StmtNode::Let(LetNode::new("n".to_string()));
    let stmtMain2 = StmtNode::Assign(AssignNode::new("n".to_string(), ExprNode::Val(Value::I32(5))));


    let stmtMain3 = StmtNode::Print(PrintNode::new(
    ExprNode::Call("factorial_recursion".to_string(), vec![
            Rc::new(ExprNode::Var("n".to_string())),
            Rc::new(ExprNode::Val(Value::Bool(false))),
            Rc::new(ExprNode::Val(Value::I32(1))),
        ])
    ));

    let stmtMain4 = StmtNode::Print(PrintNode::new(
        ExprNode::Call("factorial_loop".to_string(), vec![
            Rc::new(ExprNode::Var("n".to_string())),
        ])
    ));

    block_main.statements.push(Rc::new(stmtMain1));
    block_main.statements.push(Rc::new(stmtMain2));
    block_main.statements.push(Rc::new(stmtMain3));
    block_main.statements.push(Rc::new(stmtMain4));

    let func_main = FuncNode::new(
        "main".to_string(),
        parameters_main,
        block_main);

    program.func_nodes.push(Rc::new(func_main));



    let mut program = ProgramNode::new();

    // global variables
    let let_count = LetNode::new( "count".to_string(), Value::Nil);
    let let_help =  LetNode::new( "help".to_string(), Value::Nil);
    program.let_nodes.push(Rc::new(let_count));
    program.let_nodes.push(Rc::new(let_help));

    // add function
    let mut parameters_add = vec![];
    parameters_add.push(Parameter::new("a".to_string()));
    parameters_add.push(Parameter::new("b".to_string()));

    let mut block_add = BlockNode::new();
    let stmtAdd1 = StmtNode::Return(
        ReturnNode::new(ExprNode::Add(
            Rc::new(ExprNode::Var("a".to_string())),
            Rc::new(ExprNode::Var("b".to_string())),
        ))
    );
    block_add.statements.push(Rc::new(stmtAdd1));

    let func_add = FuncNode::new(
        "add".to_string(),
        parameters_add,
        block_add);

    program.func_nodes.push(Rc::new(func_add));

    // main function
    let mut parameters_main = vec![];
    parameters_main.push(Parameter::new("argc".to_string()));

    let mut block_main = BlockNode::new();
    let stmtMain1 = StmtNode::Let(LetNode::new("sum".to_string(), Value::Nil));
    let stmtMain2 = StmtNode::Assign(
        AssignNode::new("sum".to_string(), ExprNode::Add(
           Rc::new(ExprNode::Val(Value::I32(3))),
           Rc::new(ExprNode::Add(
               Rc::new(ExprNode::Val(Value::I32(5))),
               Rc::new(ExprNode::Val(Value::I32(7))),
           ))
        ))
    );
    let stmtMain3 = StmtNode::Print(
        PrintNode::new(ExprNode::Var("sum".to_string())));
    let stmtMain4 = StmtNode::Assign(AssignNode::new("sum".to_string(),
        ExprNode::Call("add".to_string(), vec![
            Rc::new(ExprNode::Var("sum".to_string())),
            Rc::new(ExprNode::Val(Value::I32(1)))
        ])
    ));
    let stmtMain5 = StmtNode::Print(
        PrintNode::new(ExprNode::Var("sum".to_string())));
    block_main.statements.push(Rc::new(stmtMain1));
    block_main.statements.push(Rc::new(stmtMain2));
    block_main.statements.push(Rc::new(stmtMain3));
    block_main.statements.push(Rc::new(stmtMain4));
    block_main.statements.push(Rc::new(stmtMain5));

    let func_main = FuncNode::new(
        "main".to_string(),
        parameters_main,
        block_main);

    program.func_nodes.push(Rc::new(func_main));


    Rc::new(program)
}*/


/*fn run0() {
    let rc_program = grow_ast_program0();

    let runtime = Machine::new(rc_program);
    runtime.run();
}*/

use clap::Parser;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {

    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");


    let args = Args {
        pattern,
        path: std::path::PathBuf::from(path),
    };

    println!(
"command(s):
    t (tokenize)
    p (parse)
    e (execute)
    d (debug)
");

    let _contents = fs::read_to_string(args.path).expect("Should have been able to read the file");

    runNeg1(_contents);
}
