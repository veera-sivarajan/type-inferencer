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

fn left_occurs(rhs: &Term, left: &Term) -> bool {
    match rhs {
        Term::Arrow(ArrowType { domain, range }) => {
            left_occurs(domain, left) || left_occurs(range, left)
        }
        _ => left == rhs,
    }
}

fn replace(term: &Term, left: &Term, right: &Term) -> Term {
    match term {
        Term::Arrow(ArrowType { domain, range }) => Term::Arrow(ArrowType {
            domain: Box::new(replace(domain, left, right)),
            range: Box::new(replace(range, left, right)),
        }),
        _ => {
            if term == left {
                right.clone()
            } else {
                term.clone()
            }
        }
    }
}

fn left_replace(subs: &mut [Substitution], left: &Term, right: &Term) -> Vec<Substitution> {
    if subs.len() < 2 {
        return vec![];
    }
    
    let (first, rest) = subs.split_at_mut(1);
    let first = first.first().unwrap().clone();
    
    let var = first.var;
    let is = replace(&first.is, left, right);
    let mut result = vec![Substitution { var, is }];
    result.extend(left_replace(rest, left, right));
    result
}

fn extend_replace(left: &Term, right: &Term, subst: &mut [Substitution]) -> Vec<Substitution> {
    if !left_occurs(right, left) {
        let mut result = vec![Substitution { var: left.clone(), is: right.clone() }];
        result.extend(left_replace(subst, left, right));
        result
    } else {
        panic!("Cycle in substitution.")
    }
}

fn lookup(term: &Term, subs: &[Substitution]) -> Option<Term> {
    for s in subs {
        if s.var == *term {
            return Some(s.is.clone());
        }
    }
    None
}

fn unify(consts: &mut Vec<Constraint>, subst: &mut [Substitution]) -> Vec<Substitution> {
    if consts.len() < 2 {
        return subst.to_vec();
    }

    let (first, rest) = consts.split_at(1);

    let first = first.first().unwrap();

    let left = first.lhs.clone();
    let right = first.rhs.clone();
    match &left {
        Term::Var(c) => {
            if let Some(bound) = lookup(&left, subst) {
                let mut new_consts = vec![Constraint::new(bound, right)];
                new_consts.extend(rest.to_vec());
                unify(&mut new_consts, subst)
            } else {
                let mut result = extend_replace(&left, &right, subst);
                unify(&mut rest.to_vec(), &mut result)
            }
        }
        _ => todo!(),
    }
}
