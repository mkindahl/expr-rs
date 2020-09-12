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

criterion_group!(benches, bench_simple);
criterion_main!(benches);
