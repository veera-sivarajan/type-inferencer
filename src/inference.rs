use crate::types::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Expr(Expr),
    Var(char),
    Num,
    Arrow(ArrowType),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrowType {
    domain: Box<Term>,
    range: Box<Term>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Constraint {
    lhs: Term,
    rhs: Term,
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

fn extend_replace(left: &Term, right: &Term, subst: Vec<Substitution>) -> Vec<Substitution> {
    todo!()
}

fn unify(consts: &mut Vec<Constraint>, subst: &mut Vec<Substitution>) -> Vec<Substitution> {
    if consts.len() < 2 {
        return subst.to_vec();
    }

    let (first, rest) = consts.split_at(1);

    let first = first.first().unwrap();

    let left = first.lhs.clone();
    let right = first.rhs.clone();
    match &left {
        Term::Var(c) => {
            if subst.contains(&left) {
                let mut new_consts = vec![Constraint::new(Term::Var(*c), right)];
                new_consts.extend(rest.to_vec());
                return unify(&mut new_consts, subst);
            } else {
                let mut result = extend_replace(&left, &right, subst.to_vec());
                return unify(&mut rest.to_vec(), &mut result);
            }
        }
        _ => todo!(),
    }
}
