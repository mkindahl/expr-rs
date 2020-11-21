#[macro_use]
extern crate criterion;
extern crate expr;

use criterion::Criterion;
use expr::eval;
use std::collections::HashMap;

fn bench_simple(c: &mut Criterion) {
    c.bench_function("simple", |b| {
        b.iter(|| eval("3+3*5/(3*3)", &HashMap::new()))
    });
}

fn bench_vars(c: &mut Criterion) {
    let mut vars: HashMap<String, f64> = HashMap::new();
    vars.insert("x".to_string(), 12.0);
    vars.insert("y".to_string(), 3.14);
    c.bench_function("vars", |b| b.iter(|| eval("3+3*x/(3*y)", &vars)));
}

criterion_group!(benches, bench_simple, bench_vars);
criterion_main!(benches);
