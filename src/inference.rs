use crate::types::*;

enum Term {
    TExpr(Expr),
    TVar(char),
    TNum,
    TArrow(ArrowType),
}

struct ArrowType {
    domain: Box<Term>,
    range: Box<Term>,
}

struct Constraint {
    lhs: Term,
    rhs: Term,
}

fn cons_gen(expr: &Expr) -> Vec<Constraint> {
    todo!()
}
