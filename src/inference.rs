use crate::types::*;
use std::fmt;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
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

    pub fn make_arrow(domain: &Term, range: &Term) -> Self {
        Term::Arrow(ArrowType {
            domain: Box::new(domain.clone()),
            range: Box::new(range.clone()),
        })
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct ArrowType {
    domain: Box<Term>,
    range: Box<Term>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Substitution {
    var: Term,
    is: Term,
}

impl Substitution {
    pub fn new(var: &Term, is: &Term) -> Self {
        Self {
            var: var.clone(),
            is: is.clone(),
        }
    }
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

pub fn infer_types(expr: &Expr) -> Vec<Substitution> {
    let mut cons = generate_constraints(expr);
    for c in &cons {
        println!("Constraint: {c}");
    }
    let mut subs = vec![];
    unify(&mut cons, &mut subs)
}

fn generate_constraints(expr: &Expr) -> Vec<Constraint> {
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
            let mut left_constraint = generate_constraints(left);
            let right_constraint = generate_constraints(right);
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
            let mut body_constraint = generate_constraints(body);
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
        Expr::Call(CallExp {
            caller: function,
            callee: args,
        }) => {
            let mut f_constraint = generate_constraints(function);
            let a_constraint = generate_constraints(args);
            let consequent = vec![Constraint::new(
                Term::Expr(*function.clone()),
                Term::Arrow(ArrowType {
                    domain: Box::new(Term::Expr(*args.clone())),
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

fn occurs_check(left: &Term, right: &Term) -> bool {
    match left {
        Term::Arrow(ArrowType { domain, range }) => {
            occurs_check(left, domain) || occurs_check(left, range)
        }
        _ => left == right,
    }
}

fn replace(left: &Term, term: &Term, right: &Term) -> Term {
    match term {
        Term::Arrow(ArrowType { domain, range }) => {
            Term::Arrow(ArrowType {
                domain: Box::new(replace(left, domain, right)),
                range: Box::new(replace(left, range, right)),
            })
        }
        _ => if left == term {
            right.clone()
        } else {
            term.clone()
        }
    }
}

fn replace_all(
    left: &Term,
    right: &Term,
    consts: &mut [Constraint],
    subst: &mut [Substitution],
) {
    if !occurs_check(left, right) {
        for c in consts.iter_mut() {
            c.lhs = replace(left, &c.lhs, right);
            c.rhs = replace(left, &c.rhs, right);
        }

        for s in subst.iter_mut() {
            s.var = replace(left, &s.var, right);
            s.is = replace(left, &s.is, right);
        }
    } else {
        panic!("Occurs check failed.");
    }
}

fn unify(
    consts: &mut Vec<Constraint>,
    subs: &mut Vec<Substitution>,
) -> Vec<Substitution> {
    if consts.is_empty() {
        subs.to_vec()
    } else {
        let (first, rest) = consts.split_at_mut(1);
        let first = first.first().unwrap();

        let left = &first.lhs;
        let right = &first.rhs;

        if left == right {
            unify(&mut rest.to_vec(), subs)
        } else if left.is_ident() {
            let mut new_rest = rest.to_vec();
            replace_all(left, right, &mut new_rest, subs);
            subs.push(Substitution::new(left, right));
            return unify(&mut new_rest, subs);
        } else if right.is_ident() {
            let mut new_rest = rest.to_vec();
            replace_all(right, left, &mut new_rest, subs);
            subs.push(Substitution::new(right, left));
            return unify(&mut new_rest, subs);
        } else {
            match (left, right) {
                (
                    Term::Arrow(ArrowType {
                        domain: d_one,
                        range: r_one,
                    }),
                    Term::Arrow(ArrowType {
                        domain: d_two,
                        range: r_two,
                    }),
                ) => {
                    let mut new_rest = rest.to_vec();
                    new_rest.extend(vec![
                        Constraint::new(*d_one.clone(), *d_two.clone()),
                        Constraint::new(*r_one.clone(), *r_two.clone()),
                    ]);
                    return unify(&mut new_rest.to_vec(), subs);
                }
                _ => {
                    for sub in subs {
                        println!("Found: {sub}");
                    }
                    let msg = format!("{left} and {right} do not unify.");
                    panic!("{msg}");
                }
            }
        }
    }
}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.lhs, self.rhs)
    }
}

impl fmt::Display for Substitution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} :: {}", self.var, self.is)
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Var(c) => write!(f, "Var({c})"),
            Term::Num => write!(f, "Number"),
            Term::Arrow(a_type) => {
                write!(f, "|{} -> {}|", a_type.domain, a_type.range)
            }
            Term::Expr(e) => write!(f, "{e}"),
        }
    }
}
