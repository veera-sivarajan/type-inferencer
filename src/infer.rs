use crate::types::*;

use std::collections::HashMap;

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
        state.unify();
        state.substitutions.clone()
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

    fn unify(&mut self) {
        todo!()
    }
}
