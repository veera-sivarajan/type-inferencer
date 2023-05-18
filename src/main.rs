mod inference;
mod types;

use crate::inference::*;
use crate::types::*;

fn main() {
    let add1 = Expr::Number(1) + Expr::Number(3);
    let add2 = Expr::Number(2) + Expr::Number(4);
    let exp = Expr::Conditional(IfExp::new(true.into(), add1, add2));
    println!("Input: {exp}");
    let subs = infer_types(&exp);
    for s in subs {
        println!("Sub: {s}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(output: &[Substitution], expected: &[Substitution]) -> bool {
        !expected.iter().any(|e| !output.contains(e))
    }

    #[test]
    fn test_number() {
        let exp = Expr::Number(1);
        let subs = Substitution::new(Term::Expr(exp.clone()), Term::Num);
        assert!(test(&infer_types(&exp), &[subs]))
    }

    #[test]
    fn test_binary() {
        let exp = Expr::Number(1) + Expr::Number(2);
        let subs = vec![
            Substitution::new(Term::Expr(Expr::Number(1)), Term::Num),
            Substitution::new(Term::Expr(Expr::Number(2)), Term::Num),
            Substitution::new(Term::Expr(exp.clone()), Term::Num),
        ];
        assert!(test(&infer_types(&exp), &subs))
    }

    #[test]
    fn test_function() {
        let l = Expr::Variable('x');
        let r = Expr::Number(2);
        let add = l.clone() + r;
        let f = Expr::Function(FunExp {
            argument: Box::new(l), // a = x
            arg_type: Type::Number,
            body: Box::new(add.clone()), // x + 2
        });
        let subs = vec![
            Substitution::new(
                Term::Expr(Expr::Variable('x')),
                Term::Num,
            ),
            Substitution::new(Term::Expr(Expr::Number(2)), Term::Num),
            Substitution::new(Term::Expr(add), Term::Num),
            Substitution::new(
                Term::Expr(f.clone()),
                Term::make_arrow(Term::Num, Term::Num),
            ),
        ];
        let result = infer_types(&f);
        for r in &result {
            println!("Output: {r}");
        }
        assert!(test(&result, &subs))
    }

    #[test]
    fn test_function_call() {
        let a = Expr::Variable('x');
        let r = Expr::Number(2);
        let l = Expr::Variable('x');
        let n = l + r;
        let f = Expr::Function(FunExp {
            argument: Box::new(a), // a = x
            arg_type: Type::Number,
            body: Box::new(n.clone()), // x + 2
        });
        let arg1 = Expr::Number(10);
        let c1 = Expr::Call(CallExp {
            caller: Box::new(f.clone()),
            callee: Box::new(arg1),
        }); // ((lambda(x) x + 2) (5))

        let subs = vec![
            Substitution::new(
                Term::Expr(Expr::Variable('x')),
                Term::Num,
            ),
            Substitution::new(Term::Expr(Expr::Number(2)), Term::Num),
            Substitution::new(Term::Expr(n), Term::Num),
            Substitution::new(
                Term::Expr(f),
                Term::make_arrow(Term::Num, Term::Num),
            ),
            Substitution::new(Term::Expr(Expr::Number(10)), Term::Num),
            Substitution::new(Term::Expr(c1.clone()), Term::Num),
        ];
        let result = infer_types(&c1);
        for r in &result {
            println!("Output: {r}");
        }
        assert!(test(&result, &subs))
    }

    #[test]
    fn test_function_parameters() {
        let two = Expr::Number(2);
        let five = Expr::Number(5);
        let var_x = Expr::Variable('x');

        let call_five = Expr::Call(CallExp {
            caller: Box::new(var_x),
            callee: Box::new(five),
        }); // x(5)

        let add = call_five.clone() + two;
        let first_lambda = Expr::Function(FunExp {
            argument: Box::new(Expr::Variable('x')),
            arg_type: Type::Number,
            body: Box::new(add.clone()),
        }); // (lambda(x) x(5) + 2)

        let add_five = Expr::Variable('y') + Expr::Number(5);
        let second_lambda = Expr::Function(FunExp {
            argument: Box::new(Expr::Variable('y')),
            arg_type: Type::Number,
            body: Box::new(add_five.clone()),
        }); // (lambda(y) y + 5)

        let c1 = Expr::Call(CallExp {
            caller: Box::new(first_lambda.clone()),
            callee: Box::new(second_lambda.clone()),
        }); // (lambda(x) x(5) + 2)((lambda(x) x + 5))

        let subs = vec![
            Substitution::new(
                Term::Expr(Expr::Variable('x')),
                Term::make_arrow(Term::Num, Term::Num),
            ),
            Substitution::new(Term::Expr(Expr::Number(5)), Term::Num),
            Substitution::new(Term::Expr(Expr::Number(2)), Term::Num),
            Substitution::new(Term::Expr(call_five), Term::Num),
            Substitution::new(Term::Expr(add), Term::Num),
            Substitution::new(
                Term::Expr(first_lambda),
                Term::make_arrow(
                    Term::make_arrow(Term::Num, Term::Num),
                    Term::Num,
                ),
            ),
            Substitution::new(
                Term::Expr(Expr::Variable('y')),
                Term::Num,
            ),
            Substitution::new(Term::Expr(add_five), Term::Num),
            Substitution::new(
                Term::Expr(second_lambda),
                Term::make_arrow(Term::Num, Term::Num),
            ),
            Substitution::new(Term::Expr(c1.clone()), Term::Num),
        ];
        let result = infer_types(&c1);
        for r in &result {
            println!("Output: {r}");
        }
        assert!(test(&result, &subs))
    }

    #[test]
    #[should_panic]
    fn test_incorrect_argument() {
        let two = Expr::Number(2);
        let five = Expr::Number(5);
        let var_x = Expr::Variable('x');

        let call_five = Expr::Call(CallExp {
            caller: Box::new(var_x),
            callee: Box::new(five),
        }); // x(5)

        let add = call_five + two; // x(5) + 2

        let first_lambda = Expr::Function(FunExp {
            argument: Box::new(Expr::Variable('x')),
            arg_type: Type::Number,
            body: Box::new(add),
        }); // (lambda(x) x(5) + 2)

        let number = Expr::Number(2);
        let c1 = Expr::Call(CallExp {
            caller: Box::new(first_lambda),
            callee: Box::new(number),
        }); // (lambda(x) x(5) + 2)(2)

        let _ = infer_types(&c1);
    }

    #[test]
    fn test_conditional() {
        let exp =
            Expr::Conditional(IfExp::new(true.into(), 1.into(), 2.into()));
        let subs = vec![
            Substitution::new(Term::Expr(Expr::Bool(true)), Term::Bool),
            Substitution::new(Term::Expr(Expr::Number(1)), Term::Num),
            Substitution::new(Term::Expr(Expr::Number(2)), Term::Num),
            Substitution::new(Term::Expr(exp.clone()), Term::Num),
        ];
        let result = infer_types(&exp);
        assert!(test(&result, &subs))
    }

    #[test]
    #[should_panic]
    fn test_incorrect_condition() {
        let exp =
            Expr::Conditional(IfExp::new(1.into(), 1.into(), 2.into()));
        infer_types(&exp);
    }

    #[test]
    #[should_panic]
    fn test_mismatched_branch_types() {
        let exp = Expr::Conditional(IfExp::new(
            true.into(),
            false.into(),
            2.into(),
        ));
        infer_types(&exp);
    }
}
