#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::symbols::Symbols;
use crate::value::Value;

#[derive(Debug, Clone)]
pub struct ProgramNode {
    pub symbols: Rc<RefCell<Symbols>>,
    pub let_nodes: Vec<Rc<LetNode>>,
    pub func_nodes: Vec<Rc<FuncNode>>,
}

impl ProgramNode {
    pub fn new() -> ProgramNode {
        ProgramNode {
            symbols: Rc::new(RefCell::new(Symbols::new(None))),
            let_nodes: vec![],
            func_nodes: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncNode {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub block_node: Rc<BlockNode>,
}

impl FuncNode {

    pub fn new(name: String, parameters: Vec<Parameter>, block_node: BlockNode) -> FuncNode {
        FuncNode {
            name,
            parameters,
            block_node : Rc::new(block_node),
        }
    }

    pub fn numParameters(&self) -> usize {
        self.parameters.len()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
}

impl Parameter {
    pub fn new(name: String) -> Parameter {
        Parameter {
            name
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockNode {
    pub symbols: Rc<RefCell<Symbols>>,
    pub statements: Vec<Rc<StmtNode>>,
}

impl BlockNode {
    pub fn new() -> BlockNode {
        BlockNode {
            symbols: Rc::new(RefCell::new(Symbols::new(None))),
            statements: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StmtNode {
    If(IfNode),
    Let(LetNode),
    Assign(AssignNode),
    Return(ReturnNode),
    Print(PrintNode),
    While(WhileNode),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfNode {
    pub expr: Rc<ExprNode>,
    pub then: Rc<BlockNode>,
    pub elses: Rc<BlockNode>,
}

impl IfNode {
    pub fn new(expr: ExprNode, then: BlockNode, elses: BlockNode) -> IfNode {
        IfNode {
            expr: Rc::new(expr),
            then: Rc::new(then),
            elses: Rc::new(elses),
        }
    }
    pub fn newR(expr: Rc<ExprNode>, then: Rc<BlockNode>, elses: Rc<BlockNode>) -> IfNode {
        IfNode {
            expr,
            then,
            elses,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileNode {
    pub expr: Rc<ExprNode>,
    pub then: Rc<BlockNode>,
}

impl WhileNode {
    pub fn new(expr: ExprNode, then: BlockNode) -> WhileNode {
        WhileNode {
            expr: Rc::new(expr),
            then: Rc::new(then),
        }
    }
    pub fn newR(expr: Rc<ExprNode>, then: Rc<BlockNode>) -> WhileNode {
        WhileNode {
            expr,
            then,
        }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct LetNode {
    pub name: String,
}

impl LetNode {
    pub fn new(name: String) -> LetNode {
        LetNode {
            name,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssignNode {
    pub name: String,
    pub expr: Rc<ExprNode>,
}

impl AssignNode {
    pub fn new(name: String, expr: ExprNode) -> AssignNode {
        AssignNode {
            name,
            expr: Rc::new(expr),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrintNode {
    pub expr: Rc<ExprNode>,
}

impl PrintNode {
    pub fn new(expr: ExprNode) -> PrintNode {
        PrintNode {
            expr: Rc::new(expr),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnNode {
    pub expr: Rc<ExprNode>,
}

impl ReturnNode {
    pub fn new(expr: ExprNode) -> ReturnNode {
        ReturnNode {
            expr: Rc::new(expr),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprNode {
    Var(String),
    Val(Value),
    Not(Rc<ExprNode>),
    EQUAL(Rc<ExprNode>, Rc<ExprNode>),
    OR_BIT(Rc<ExprNode>, Rc<ExprNode>),
    AND_BIT(Rc<ExprNode>, Rc<ExprNode>),
    LT(Rc<ExprNode>, Rc<ExprNode>),
    GT(Rc<ExprNode>, Rc<ExprNode>),
    Add(Rc<ExprNode>, Rc<ExprNode>),
    Sub(Rc<ExprNode>, Rc<ExprNode>),
    Mul(Rc<ExprNode>, Rc<ExprNode>),
    Div(Rc<ExprNode>, Rc<ExprNode>),
    Call(String, Vec<Rc<ExprNode>>),
}

impl ExprNode {

}


