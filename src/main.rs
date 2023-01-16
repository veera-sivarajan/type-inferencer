mod inference;
mod types;

use crate::inference::*;
use crate::types::*;

fn main() {
    let l = Expr::Number(1.0);
    let r = Expr::Number(2.0);
    let n = Expr::Binary(BinExp::new(l, Operation::Add, r));
    println!("{:#?}", inference::cons_gen(&n));
    println!("Hello, world!");
}
