mod lib;

use std::env;
use lib::PostfixEvaluator;

fn main() {
    let mut evaluator = PostfixEvaluator::new();
    let expr = env::args()
        .skip(1)
        .fold(String::new(), |acc, arg| format!("{acc} {arg}"))
        .trim_start()
        .to_string();
    
    match evaluator.eval(&expr) {
        Ok(val) => println!("Result of calculation: {val}"),
        Err(msg) => println!("Error when calculating the expression: {msg}"),
    }
}
