use criterion::{criterion_group, criterion_main, Criterion};

fn handshake_benchmark(_c: &mut Criterion) {}

criterion_group!(benches, handshake_benchmark);
criterion_main!(benches);
