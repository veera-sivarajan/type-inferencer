mod infer;
mod inference;
mod types;

use crate::inference::*;
use crate::types::*;

fn main() {
    let a = Expr::Variable('x');
    let r = Expr::Number(2);
    let l = Expr::Variable('x');
    let n = Expr::Binary(BinExp::new(l, Operation::Add, r)); // x + 2
    let f = Expr::Function(FunExp {
        argument: Box::new(a), // a = x
        arg_type: Type::Number,
        body: Box::new(n), // x + 2
    });
    let arg1 = Expr::Number(10);
    let c1 = Expr::Call(CallExp {
        caller: Box::new(f),
        callee: Box::new(arg1),
    }); // ((lambda(x) x + 2) (5))

    println!("Input: {c1}");
    let constraints = inference::cons_gen(&c1);
    for c in &constraints {
        println!("Constraints: {c:?}");
    }
    // let mut substs = vec![];
    // let result = inference::unify(&mut constraints, &mut substs);
    // for r in result {
    //     println!("Sub: {r}");
    // }

    // let subs = infer::Types::infer(&c1);
    // for (k, v) in subs {
    //     println!("Other Constraint: {k:?} with {v:?}");
    // }
    let subs = infer::infer_types(&c1);
    for s in subs {
        println!("Sub: {s}");
    }
}
