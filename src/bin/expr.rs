extern crate expr;

use expr::eval;
use std::collections::HashMap;
use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: expr <expression> [ <variable>=<value> ... ]");
    } else {
        let expr = &args[1];
        let mut map = HashMap::new();
        for assign in &args[2..] {
            let parts: Vec<&str> = assign.splitn(2, '=').collect();
            map.insert(parts[0].to_string(), parts[1].parse::<f64>()?);
        }
        println!("{}", eval(expr.as_str(), &map)?);
    }
    Ok(())
}
