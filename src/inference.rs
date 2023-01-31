use crate::types::*;
use std::fmt;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Term {
    Expr(Expr), // variable
    Var(char),  // variable
    Num,        // constant
    Bool,
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
    let mut cons = vec![];
    generate_constraints(expr, &mut cons);
    for c in &cons {
        println!("Constraint: {c}");
    }
    unify(&mut cons, &mut vec![])
}

fn generate_constraints(expr: &Expr, constraints: &mut Vec<Constraint>) {
    match expr {
        Expr::Number(_) => {
            // When the expression is a number, we expect the type
            // of the expression to be numeric:
            constraints.push(Constraint {
                lhs: Term::Expr(expr.clone()),
                rhs: Term::Num,
            });
        }
        Expr::Variable(s) => {
            constraints.push(Constraint {
                lhs: Term::Expr(expr.clone()),
                rhs: Term::Var(*s),
            });
        }
        Expr::Bool(_) => {
            constraints.push(Constraint {
                lhs: Term::Expr(expr.clone()),
                rhs: Term::Bool,
            });
        }
        Expr::Binary(BinExp {
            left,
            operator: _,
            right,
        }) => {
            generate_constraints(left, constraints);
            generate_constraints(right, constraints);
            let consequent = vec![
                Constraint::new(Term::Expr(*left.clone()), Term::Num),
                Constraint::new(Term::Expr(*right.clone()), Term::Num),
                Constraint::new(Term::Expr(expr.clone()), Term::Num),
            ];
            constraints.extend(consequent);
        }
        Expr::Conditional(IfExp {
            condition,
            then,
            elze,
        }) => {
            generate_constraints(condition, constraints);
            generate_constraints(then, constraints);
            generate_constraints(elze, constraints);
            let rest = vec![
                Constraint::new(
                    Term::Expr(*condition.clone()),
                    Term::Bool,
                ),
                Constraint::new(
                    Term::Expr(expr.clone()),
                    Term::Expr(*then.clone()),
                ),
                Constraint::new(
                    Term::Expr(expr.clone()),
                    Term::Expr(*elze.clone()),
                ),
            ];
            constraints.extend(rest);
        }
        Expr::Function(FunExp {
            argument,
            arg_type: _,
            body,
        }) => {
            generate_constraints(body, constraints);
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
            constraints.extend(consequent);
        }
        Expr::Call(CallExp {
            caller: function,
            callee: args,
        }) => {
            generate_constraints(function, constraints);
            generate_constraints(args, constraints);
            let consequent = vec![Constraint::new(
                Term::Expr(*function.clone()),
                Term::Arrow(ArrowType {
                    domain: Box::new(Term::Expr(*args.clone())),
                    range: Box::new(Term::Expr(expr.clone())),
                }),
            )];
            constraints.extend(consequent);
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
        _ => {
            if left == term {
                right.clone()
            } else {
                term.clone()
            }
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
            replace_all(left, right, rest, subs);
            subs.push(Substitution::new(left, right));
            unify(&mut rest.to_vec(), subs)
        } else if right.is_ident() {
            replace_all(right, left, rest, subs);
            subs.push(Substitution::new(right, left));
            unify(&mut rest.to_vec(), subs)
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
                    unify(&mut new_rest, subs)
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
            Term::Var(c) => write!(f, "{c}"),
            Term::Num => write!(f, "Number"),
            Term::Bool => write!(f, "Bool"),
            Term::Arrow(a_type) => {
                write!(f, "{} -> {}", a_type.domain, a_type.range)
            }
            Term::Expr(e) => write!(f, "{e}"),
        }
    }
}
