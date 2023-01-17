use crate::types::*;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Term {
    Expr(Expr),       // variable
    Var(char),        // variable
    Num,              // constant
    Arrow(ArrowType), // function application
}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Term::Expr(a), Term::Expr(b)) => a == b,
            (Term::Var(a), Term::Var(b)) => a == b,
            (Term::Num, Term::Num) => true,
            (Term::Arrow(a), Term::Arrow(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Term {}

impl Term {
    fn is_ident(&self) -> bool {
        matches!(self, Term::Expr(_) | Term::Var(_))
    }

    fn is_func(&self) -> bool {
        matches!(self, Term::Arrow(_))
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Var(c) => write!(f, "{c}"),
            Term::Num => write!(f, "Number"),
            Term::Arrow(a_type) => {
                write!(f, "{} -> {}", a_type.domain, a_type.range)
            }
            Term::Expr(e) => write!(f, "{e}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArrowType {
    domain: Box<Term>,
    range: Box<Term>,
}

impl PartialEq for ArrowType {
    fn eq(&self, other: &Self) -> bool {
        self.domain == other.domain && self.range == other.range
    }
}

impl Eq for ArrowType {}

#[derive(Debug, Clone, PartialEq)]
pub struct Constraint {
    lhs: Term,
    rhs: Term,
}


impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.lhs, self.rhs)
    }
}

impl Constraint {
    fn new(lhs: Term, rhs: Term) -> Self {
        Constraint { lhs, rhs }
    }
}

pub fn cons_gen(expr: &Expr) -> Vec<Constraint> {
    match expr {
        Expr::Number(_) => {
            // When the expression is a number, we expect the type
            // of the expression to be numeric:
            vec![Constraint {
                lhs: Term::Expr(expr.clone()),
                rhs: Term::Num,
            }]
        }
        Expr::Variable(s) => {
            vec![Constraint {
                lhs: Term::Expr(expr.clone()),
                rhs: Term::Var(*s),
            }]
        }
        Expr::Binary(BinExp {
            left,
            operator: _,
            right,
        }) => {
            let mut left_constraint = cons_gen(left);
            let right_constraint = cons_gen(right);
            let consequent = vec![
                Constraint::new(Term::Expr(*left.clone()), Term::Num),
                Constraint::new(Term::Expr(*right.clone()), Term::Num),
                Constraint::new(Term::Expr(expr.clone()), Term::Num),
            ];
            left_constraint.extend(right_constraint);
            left_constraint.extend(consequent);
            left_constraint
        }
        Expr::Function(FunExp {
            argument,
            arg_type: _,
            body,
        }) => {
            let mut body_constraint = cons_gen(body);
            let Expr::Variable(a) = **argument else {
                panic!("Function argument is not a variable.");
            };
            let consequent = vec![Constraint {
                lhs: Term::Expr(expr.clone()),
                rhs: Term::Arrow(ArrowType {
                    domain: Box::new(Term::Var(a)),
                    range: Box::new(Term::Expr(*body.clone())),
                }),
            }];
            body_constraint.extend(consequent);
            body_constraint
        }
        Expr::Call(CallExp { caller, callee }) => {
            let mut f_constraint = cons_gen(caller);
            let a_constraint = cons_gen(callee);
            let consequent = vec![Constraint::new(
                Term::Expr(*caller.clone()),
                Term::Arrow(ArrowType {
                    domain: Box::new(Term::Expr(*callee.clone())),
                    range: Box::new(Term::Expr(expr.clone())),
                }),
            )];
            f_constraint.extend(a_constraint);
            f_constraint.extend(consequent);
            f_constraint
        }
        _ => todo!(),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Substitution {
    var: Term,
    is: Term,
}

impl Substitution {
    fn new(var: &Term, is: &Term) -> Self {
        Self {
            var: var.clone(),
            is: is.clone(),
        }
    }
}

impl fmt::Display for Substitution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} :: {}", self.var, self.is)
    }
}

fn occurs_check(
    left: &Term,
    right: &Term,
) -> bool {
    match left {
        Term::Arrow(ArrowType { domain, range }) => {
            occurs_check(left, domain) || occurs_check(left, range)
        }
        _ => left == right,
    }
}

fn replace_all(
    left: &Term,
    right: &Term,
    consts: &mut Vec<Constraint>,
    subst: &mut Vec<Substitution>,
) {
    if !occurs_check(left, right) {
        println!("Replacing {left} with {right}");
        for c in consts {
            if c.lhs == *left {
                c.lhs = right.clone();
            } else if c.rhs == *left {
                c.rhs = right.clone();
            }
        }

        for sub in subst {
            if sub.is == *left {
                sub.is = right.clone();
            } else if sub.var == *left {
                sub.var = right.clone();
            }
        }
    } else {
        panic!("occurs_check failed.");
    }
}

pub fn unify(
    consts: &mut Vec<Constraint>,
    subst: &mut Vec<Substitution>,
) -> Vec<Substitution> {
    if consts.is_empty() {
        subst.to_vec()
    } else {
        let (first, rest) = consts.split_at_mut(1);
        let first = first.first().unwrap();

        let left = first.lhs.clone();
        let right = first.rhs.clone();

        if left == right {
            subst.to_vec()
        } else if left.is_ident() {
            replace_all(&left, &right, &mut rest.to_vec(), subst);
            subst.push(Substitution::new(&left, &right));
            return unify(&mut rest.to_vec(), subst);
        } else if right.is_ident() {
            replace_all(&right, &left, &mut rest.to_vec(), subst);
            subst.push(Substitution::new(&right, &left));
            return unify(&mut rest.to_vec(), subst);
        } else if left.is_func() && right.is_func() {
            match (left, right) {
                (Term::Arrow(func_a), Term::Arrow(func_b)) => {
                    if func_a == func_b {
                        consts.push(Constraint::new(
                            *func_a.domain,
                            *func_b.domain,
                        ));
                    }
                    return unify(&mut consts.to_vec(), subst);
                }
                _ => unreachable!(),
            }
        } else {
            let msg = format!("{left} and {right} do not unify.");
            panic!("{msg}");
        }
    }
}
