mod inference;
mod types;

use crate::inference::*;
use crate::types::*;

fn main() {
    let a = Expr::Variable('x');
    let r = Expr::Number(2.0);
    let l = Expr::Variable('x'); 
    let n = Expr::Binary(BinExp::new(l, Operation::Add, r));
    let f = Expr::Function(FunExp {
        argument: Box::new(a),
        arg_type: Type::Number,
        body: Box::new(n),
    });
    let arg1 = Expr::Number(5.0);
    let c1 = Expr::Call(CallExp {
        caller: Box::new(f.clone()),
        callee: Box::new(arg1),
    });

    // let c2 = Expr::Call(CallExp {
    //     caller: Box::new(f.clone()),
    //     callee: Box::new(c1),
    // });

    println!("Input: {c1}");
    let mut constraints = inference::cons_gen(&c1);
    for c in &constraints {
        println!("Constraints: {c:?}");
    }
    let mut substs = vec![];
    let result = inference::unify(&mut constraints, &mut substs);
    for r in result {
        println!("{r}");
    }
}
