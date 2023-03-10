use std::fmt;
use std::ops::Add;

#[derive(Eq, Debug, Hash, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Type {
    Number,
    String,
    Bool,
    Function(FunType),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Expr {
    Number(i64),
    // String(String),
    Bool(bool),
    Variable(char),
    Binary(BinExp),
    Conditional(IfExp),
    Function(FunExp),
    Call(CallExp),
}

impl From<i64> for Expr {
    fn from(num: i64) -> Self {
        Expr::Number(num)
    }
}

impl From<bool> for Expr {
    fn from(value: bool) -> Self {
        Expr::Bool(value)
    }
}

impl Add for Expr {
    type Output = Expr;

    fn add(self, other: Self) -> Self::Output {
        Expr::Binary(BinExp::new(self, Operation::Add, other))
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{n}"),
            Expr::Binary(e) => write!(f, "{e}"),
            Expr::Function(e) => write!(f, "{e}"),
            Expr::Call(e) => write!(f, "{e}"),
            Expr::Variable(c) => write!(f, "{c}"),
            Expr::Conditional(e) => write!(f, "{e}"),
            Expr::Bool(b) => write!(f, "{b}"),
            _ => todo!(),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BinExp {
    pub left: Box<Expr>,
    pub operator: Operation,
    pub right: Box<Expr>,
}

impl fmt::Display for BinExp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
    }
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

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
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

impl fmt::Display for IfExp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "if {} {{{}}} else {{{}}}",
            self.condition, self.then, self.elze
        )
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
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

impl fmt::Display for FunExp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(lambda({}) {})", self.argument, self.body)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CallExp {
    pub caller: Box<Expr>,
    pub callee: Box<Expr>,
}

impl fmt::Display for CallExp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Call<{}({})>", self.caller, self.callee)
    }
}

impl CallExp {
    pub fn new(ler: Expr, lee: Expr) -> Self {
        Self {
            caller: Box::new(ler),
            callee: Box::new(lee),
        }
    }
}
