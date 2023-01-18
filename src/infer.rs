use crate::types::*;

use std::collections::HashMap;
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

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
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
        let mut state = Self::default();
        state.generate_constraints(expr);
        for (k, v) in &state.constraints {
            println!("{k} --> {v}");
        }
        state.unify()
    }

    fn generate_constraints(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(_) => {
                // When the expression is a number, we expect the type
                // of the expression to be numeric:
                self.constraints
                    .insert(Term::Expr(expr.clone()), Term::Num);
            }
            Expr::Variable(s) => {
                self.constraints
                    .insert(Term::Expr(expr.clone()), Term::Var(*s));
            }
            Expr::Binary(BinExp {
                left,
                operator: _,
                right,
            }) => {
                self.generate_constraints(left);
                self.generate_constraints(right);
                self.constraints
                    .insert(Term::Expr(*left.clone()), Term::Num);
                self.constraints
                    .insert(Term::Expr(*right.clone()), Term::Num);
                self.constraints
                    .insert(Term::Expr(expr.clone()), Term::Num);
            }
            Expr::Function(FunExp {
                argument,
                arg_type: _,
                body,
            }) => {
                self.generate_constraints(body);
                let Expr::Variable(a) = **argument else {
                    panic!("Function argument is not a variable.");
                };
                self.constraints.insert(
                    Term::Expr(expr.clone()),
                    Term::Arrow(ArrowType {
                        domain: Box::new(Term::Var(a)),
                        range: Box::new(Term::Expr(*body.clone())),
                    }),
                );
            }
            Expr::Call(CallExp {
                caller: function,
                callee: args,
            }) => {
                self.generate_constraints(function);
                self.generate_constraints(args);
                self.constraints.insert(
                    Term::Expr(*function.clone()),
                    Term::Arrow(ArrowType {
                        domain: Box::new(Term::Expr(*args.clone())),
                        range: Box::new(Term::Expr(expr.clone())),
                    }),
                );
            }
            _ => todo!(),
        }
    }

    fn occurs_check(&self, left: &Term, right: &Term) -> bool {
        match left {
            Term::Arrow(ArrowType { domain, range }) => {
                self.occurs_check(left, domain)
                    || self.occurs_check(left, range)
            }
            _ => left == right,
        }
    }

    fn get_replacement(
        &self,
        left: Term,
        substitutions: &HashMap<Term, Term>,
    ) -> Option<Term> {
        for (k, v) in substitutions {
            if let Term::Arrow(func) = &k {
                if *func.domain == left {
                    return Some(v.clone());
                }

                if *func.range == left {
                    return Some(v.clone());
                }
            }

            if let Term::Arrow(func) = &v {
                if *func.domain == left {
                    return Some(k.clone());
                }

                if *func.range == left {
                    return Some(k.clone());
                }
            }

            if &left == k {
                return Some(v.clone());
            }

            if &left == v {
                return Some(k.clone());
            }
        }
        Some(left)
    }

    fn unify_helper(
        &self,
        left: &Term,
        right: &Term,
        substitutions: &mut Substitutions,
    ) {
        if !self.occurs_check(left, right) {
            if let Some(left) = self.get_replacement(left.clone(), substitutions) {
                substitutions.insert(left, right.clone());
            }
        }
    }

    fn unify_assist(&self, left: &Term, right: &Term, substitutions: &mut Substitutions) {
        if left == right {
        } else if left.is_ident() {
            self.unify_helper(left, right, substitutions);
        } else if right.is_ident() {
            self.unify_helper(right, left, substitutions);
        } else {
            match (left, right) {
                (Term::Arrow(a_one), Term::Arrow(a_two)) => {
                    let (d_one, d_two) =
                        (a_one.domain.clone(), a_two.domain.clone());
                    let (r_one, r_two) =
                        (a_one.range.clone(), a_two.range.clone());
                    self.unify_assist(
                        &d_one,
                        &d_two,
                        substitutions,
                    );
                    self.unify_assist(
                        &r_one,
                        &r_two,
                        substitutions,
                    );
                }
                _ => {
                    let msg =
                        format!("{left} and {right} do not unify.");
                    panic!("{msg}");
                }
            }
        }
    }

    fn unify(&mut self) -> Substitutions {
        let mut substitutions = HashMap::new();
        for (left, right) in &self.constraints {
            self.unify_assist(left, right, &mut substitutions);
            // if left == right {
            //     continue;
            // } else if left.is_ident() {
            //     self.unify_helper(left, right, &mut substitutions);
            // } else if right.is_ident() {
            //     self.unify_helper(right, left, &mut substitutions);
            // } else {
            //     match (left, right) {
            //         (Term::Arrow(a_one), Term::Arrow(a_two)) => {
            //             let (d_one, d_two) =
            //                 (a_one.domain.clone(), a_two.domain.clone());
            //             let (r_one, r_two) =
            //                 (a_one.range.clone(), a_two.range.clone());
            //             self.unify_helper(
            //                 &d_one,
            //                 &d_two,
            //                 &mut substitutions,
            //             );
            //             self.unify_helper(
            //                 &r_one,
            //                 &r_two,
            //                 &mut substitutions,
            //             );
            //         }
            //         _ => {
            //             let msg =
            //                 format!("{left} and {right} do not unify.");
            //             panic!("{msg}");
            //         }
            //     }
            // }
        }
        return substitutions.clone();
    }
}
