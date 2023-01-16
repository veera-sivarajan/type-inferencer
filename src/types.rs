use std::fmt;

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum Operation {
    Add,
    Sub,
    Less,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Sub => write!(f, "-"),
            Operation::Less => write!(f, "<"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Number,
    String,
    Bool,
    Function(FunType),
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunType {
    pub input: Box<Type>,
    pub output: Box<Type>,
}

impl FunType {
    pub fn new(input: Type, output: Type) -> Self {
        Self {
            input: Box::new(input),
            output: Box::new(output),
        }
    }
}

#[derive(Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Bool(bool),
    Variable(char),
    Binary(BinExp),
    Conditional(IfExp),
    Function(FunExp),
    Call(CallExp),
}

#[derive(Clone)]
pub struct BinExp {
    pub left: Box<Expr>,
    pub operator: Operation,
    pub right: Box<Expr>,
}

impl BinExp {
    pub fn new(left: Expr, operator: Operation, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Clone)]
pub struct IfExp {
    pub condition: Box<Expr>,
    pub then: Box<Expr>,
    pub elze: Box<Expr>,
}

impl IfExp {
    pub fn new(cond: Expr, then: Expr, elze: Expr) -> Self {
        Self {
            condition: Box::new(cond),
            then: Box::new(then),
            elze: Box::new(elze),
        }
    }
}

#[derive(Clone)]
pub struct FunExp {
    pub argument: Box<Expr>,
    pub arg_type: Type,
    pub body: Box<Expr>,
}

impl FunExp {
    pub fn new(argument: Expr, arg_type: Type, body: Expr) -> Self {
        Self {
            argument: Box::new(argument),
            arg_type,
            body: Box::new(body),
        }
    }
}

#[derive(Clone)]
pub struct CallExp {
    pub caller: Box<Expr>,
    pub callee: Box<Expr>,
}

impl CallExp {
    pub fn new(ler: Expr, lee: Expr) -> Self {
        Self {
            caller: Box::new(ler),
            callee: Box::new(lee),
        }
    }
}
