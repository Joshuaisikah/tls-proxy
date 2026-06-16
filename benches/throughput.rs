use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tls_proxy::inspect::{inspect, inspect_response};

fn bench_inspect_request(c: &mut Criterion) {
    let bytes = b"GET /api/v1/repos HTTP/1.1\r\nHost: github.com\r\nUser-Agent: curl/8.0\r\n\r\n";
    c.bench_function("inspect_request", |b| {
        b.iter(|| inspect(black_box(bytes)))
    });
}

fn bench_inspect_response(c: &mut Criterion) {
    let bytes = b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 42\r\n\r\n";
    c.bench_function("inspect_response", |b| {
        b.iter(|| inspect_response(black_box(bytes)))
    });
}

fn bench_inspect_request_no_host(c: &mut Criterion) {
    let bytes = b"GET / HTTP/1.1\r\nAccept: */*\r\n\r\n";
    c.bench_function("inspect_request_no_host", |b| {
        b.iter(|| inspect(black_box(bytes)))
    });
}

criterion_group!(benches, bench_inspect_request, bench_inspect_response, bench_inspect_request_no_host);
criterion_main!(benches);
