mod inference;
mod types;

use crate::inference::*;
use crate::types::*;

fn main() {
    // let arg1 = Expr::Number(5.0);
    // let c1 = Expr::Call(CallExp {
    //     caller: Box::new(f.clone()),
    //     callee: Box::new(arg1),
    // });

    // let c2 = Expr::Call(CallExp {
    //     caller: Box::new(f),
    //     callee: Box::new(c1),
    // });

    let r = Expr::Number(2.0);
    let l = Expr::Variable('x');
    let n = Expr::Binary(BinExp::new(l, Operation::Add, r));

    let a = Expr::Variable('x');
    let f = Expr::Function(FunExp {
        argument: Box::new(a),
        arg_type: Type::Number,
        body: Box::new(n),
    });
    let mut constraints = inference::cons_gen(&f);
    println!("Constraints: {constraints:#?}");
    let result = inference::unify(&mut constraints);
    for r in result {
        println!("{r}");
    }
}
