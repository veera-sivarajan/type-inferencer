use crate::types::*;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Expr(Expr),       // variable
    Var(char),        // variable
    Num,              // constant
    Arrow(ArrowType), // function application
}

impl Term {
    fn is_ident(&self) -> bool {
        matches!(self, Term::Expr(_) | Term::Var(_))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrowType {
    domain: Box<Term>,
    range: Box<Term>,
}

type Constraints = HashMap<Term, Term>;
type Substitutions = HashMap<Term, Term>;

#[derive(Default)]
pub struct Types {
    constraints: Constraints,
    substitutions: Substitutions,
}

impl Types {
    pub fn infer(expr: &Expr) -> Substitutions {
        todo!()
    }

    fn generate_constraints(&mut self) {
        todo!()
    }

    fn unify(&mut self) {
        todo!()
    }

}
