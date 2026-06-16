use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tls_proxy::policy::fedora::FedoraPolicy;

fn bench_policy_load(c: &mut Criterion) {
    c.bench_function("policy_load", |b| {
        b.iter(|| FedoraPolicy::load(black_box("/etc/crypto-policies/back-ends/gnutls.config")))
    });
}

fn bench_to_provider(c: &mut Criterion) {
    let policy = FedoraPolicy::load("/etc/crypto-policies/back-ends/gnutls.config");
    c.bench_function("to_provider", |b| {
        b.iter(|| policy.to_provider())
    });
}

fn bench_protocol_versions(c: &mut Criterion) {
    let policy = FedoraPolicy::load("/etc/crypto-policies/back-ends/gnutls.config");
    c.bench_function("protocol_versions", |b| {
        b.iter(|| policy.protocol_versions())
    });
}

criterion_group!(benches, bench_policy_load, bench_to_provider, bench_protocol_versions);
criterion_main!(benches);
