mod inference;
mod types;

use crate::inference::infer_types;
use crate::types::*;

fn main() {
    // let a = Expr::Variable('x');
    // let r = Expr::Number(2);
    // // let l = Expr::Variable('x');
    // let n = Expr::Binary(BinExp::new(l, Operation::Add, r)); // x + 2
    // let f = Expr::Function(FunExp {
    //     argument: Box::new(a), // a = x
    //     arg_type: Type::Number,
    //     body: Box::new(n), // x + 2
    // });
    // let l = Expr::Call(CallExp {
    //     caller: Box::new(f.clone()),
    //     callee: Box::new(Expr::Number(10)),
    // });
    // let arg1 = Expr::Number(10);
    // let c1 = Expr::Call(CallExp {
    //     caller: Box::new(f),
    //     callee: Box::new(arg1),
    // }); // ((lambda(x) x + 2) (5))

    let two = Expr::Number(2);
    let five = Expr::Number(5);
    let var_x = Expr::Variable('x');

    let call_five = Expr::Call(CallExp {
        caller: Box::new(var_x),
        callee: Box::new(five),
    });

    let add = Expr::Binary(BinExp::new(call_five, Operation::Add, two)); // x(5) + 2

    let first_lambda = Expr::Function(FunExp {
        argument: Box::new(Expr::Variable('x')),
        arg_type: Type::Number,
        body: Box::new(add),
    });

    let add_five= Expr::Binary(BinExp::new(Expr::Variable('x'), Operation::Add, Expr::Number(5))); // x(5) + 2
    let second_lambda = Expr::Function(FunExp {
        argument: Box::new(Expr::Variable('x')),
        arg_type: Type::Number,
        body: Box::new(add_five),
    });

    let c1 = Expr::Call(CallExp {
        caller: Box::new(first_lambda),
        callee: Box::new(second_lambda),
    });

    println!("Input: {c1}");
    let subs = infer_types(&c1);
    for s in subs {
        println!("Sub: {s}");
    }
}
