use crate::types::*;
use std::fmt;

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
        Expr::Call(CallExp {
            caller: function,
            callee: args,
        }) => {
            let mut f_constraint = cons_gen(function);
            let a_constraint = cons_gen(args);
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

fn occurs_check(left: &Term, right: &Term) -> bool {
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
    consts: &mut [Constraint],
    subst: &mut [Substitution],
) {
    if !occurs_check(left, right) {
        // println!("Constraints: {:#?}", consts);
        for c in consts.iter_mut() {
            if let Term::Arrow(func) = &mut c.lhs {
                if *func.domain == *left {
                    *func.domain = right.clone();
                }

                if *func.range == *left {
                    *func.range = right.clone();
                }
            }

            if let Term::Arrow(func) = &mut c.rhs {
                if *func.domain == *left {
                    *func.domain = right.clone();
                }

                if *func.range == *left {
                    *func.range = right.clone();
                }
            }

            if c.lhs == *left {
                // println!("Replacing {left} with {right}");
                c.lhs = right.clone();
            } else if c.rhs == *left {
                // println!("Replacing {left} with {right}.");
                c.rhs = right.clone();
            }
        }

        for sub in subst {
            if let Term::Arrow(func) = &mut sub.var {
                if *func.domain == *left {
                    *func.domain = right.clone();
                }

                if *func.range == *left {
                    *func.range = right.clone();
                }
            }

            if let Term::Arrow(func) = &mut sub.is {
                if *func.domain == *left {
                    *func.domain = right.clone();
                }

                if *func.range == *left {
                    *func.range = right.clone();
                }
            }
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

        let left = &first.lhs;
        let right = &first.rhs;

        if left == right {
            unify(&mut rest.to_vec(), subst)
        } else if left.is_ident() {
            let mut new_rest = rest.to_vec();
            replace_all(left, right, &mut new_rest, subst);
            subst.push(Substitution::new(left, right));
            return unify(&mut new_rest, subst);
        } else if right.is_ident() {
            let mut new_rest = rest.to_vec();
            replace_all(right, left, &mut new_rest, subst);
            subst.push(Substitution::new(right, left));
            return unify(&mut new_rest, subst);
        } else {
            match (left, right) {
                (Term::Arrow(a_one), Term::Arrow(a_two)) => {
                    let (d_one, d_two) =
                        (a_one.domain.clone(), a_two.domain.clone());
                    let (r_one, r_two) =
                        (a_one.range.clone(), a_two.range.clone());
                    let mut new_rest = rest.to_vec();
                    new_rest.extend(vec![
                        Constraint::new(*d_one, *d_two),
                        Constraint::new(*r_one, *r_two),
                    ]);
                    return unify(&mut new_rest.to_vec(), subst);
                }
                _ => {
                    let msg = format!("{left} and {right} do not unify.");
                    panic!("{msg}");
                }
            }
        }
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
