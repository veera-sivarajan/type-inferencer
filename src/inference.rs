use crate::types::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Expr(Expr), // variable
    Var(char), // variable
    Num, // constant
    Arrow(ArrowType), // function application
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

fn occurs_check(rhs: &Term, left: &Term, subs: &mut [Substitution]) -> bool {
    match rhs {
        Term::Var(_) | Term::Expr(_) if lookup(rhs, subs).is_some() => {
            let value = lookup(rhs, subs).unwrap();
            occurs_check(&value, left, subs)
        }
        Term::Arrow(ArrowType { domain, range }) => {
            occurs_check(domain, left, subs) || occurs_check(range, left, subs)
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
    if subs.is_empty() {
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
    if !occurs_check(right, left, subst) {
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

pub fn unify(consts: &mut [Constraint], subst: &mut [Substitution]) -> Vec<Substitution> {
    if consts.is_empty() {
        return subst.to_vec();
    }

    let (first, rest) = consts.split_at_mut(1);

    let first = first.first().unwrap();

    let left = first.lhs.clone();
    let right = first.rhs.clone();
    match &left {
        Term::Var(_) | Term::Expr(_) => {
            if let Some(bound) = lookup(&left, subst) {
                let mut new_consts = vec![Constraint::new(bound, right)];
                new_consts.extend(rest.to_vec());
                unify(&mut new_consts, subst)
            } else {
                let mut result = extend_replace(&left, &right, subst);
                unify(rest, &mut result)
            }
        }
        Term::Num => {
            match right {
                Term::Num => unify(rest, subst),
                _ => panic!("Unify number and something else."),
            }
        }
        Term::Arrow(ArrowType { domain, range }) => {
            match right {
                Term::Arrow(a_type) => {
                    let right_domain = a_type.domain;
                    let right_range = a_type.range;
                    let d_consts = Constraint::new(*domain.clone(), *right_domain);
                    let r_consts = Constraint::new(*range.clone(), *right_range);
                    let mut list = vec![d_consts, r_consts];
                    list.extend(consts.to_owned());
                    unify(&mut list, subst)
                }
                _ => panic!("Unify arrow and something else.")
            }
        }
    }
}
