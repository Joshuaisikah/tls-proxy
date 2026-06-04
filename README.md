# tls-proxy

A TLS-intercepting proxy built to learn [rustls](https://github.com/rustls/rustls) internals from the ground up.

This project is a deliberate learning exercise: each phase adds one real capability while forcing deep engagement with rustls APIs — `CryptoProvider`, `ServerConfig`, `ClientConfig`, the record layer, and Fedora system crypto policies.

## Goal

Build a proxy that:
1. Accepts TLS connections from clients (acts as a TLS server)
2. Forwards traffic to upstream HTTPS targets (acts as a TLS client)
3. Can inspect plaintext between the two TLS sessions
4. Applies Fedora system crypto policy (`/etc/crypto-policies`) to filter cipher suites, key exchange groups, and signature algorithms

## Feature Checklist

### Phase 1 — TCP plumbing
- [ ] CLI: `--listen <addr>` and `--target <addr>` via clap
- [ ] Accept TCP connections with tokio `TcpListener`
- [ ] Connect to upstream with `TcpStream::connect`
- [ ] Forward bytes in both directions with `copy_bidirectional`
- [ ] Baseline throughput benchmark

### Phase 2 — TLS client side
- [ ] `ClientConfig` with webpki-roots trust anchors
- [ ] Wrap upstream in `TlsConnector` via tokio-rustls
- [ ] Proxy reaches `https://google.com`
- [ ] Print negotiated cipher suite and TLS version
- [ ] Print FIPS status
- [ ] Handshake latency benchmark (100 handshakes)

### Phase 3 — TLS server side
- [ ] Self-signed cert generated at startup with rcgen
- [ ] `ServerConfig` wired up
- [ ] `TlsAcceptor` wraps the listener
- [ ] `curl --insecure` connects successfully
- [ ] Browser connects and sees the certificate
- [ ] Concurrency benchmark (max simultaneous TLS connections)

### Phase 4 — Full bridge
- [ ] Decrypt client TLS, re-encrypt to upstream
- [ ] Full bidirectional TLS bridge
- [ ] Inspect and log both connections
- [ ] End-to-end latency overhead benchmark vs direct connection

### Phase 5 — Fedora crypto policy
- [ ] Parse `/etc/crypto-policies/back-ends/gnutls.config`
- [ ] Map GnuTLS names → rustls cipher suites / kx groups / sig algs
- [ ] Build a filtered `CryptoProvider` from the active policy
- [ ] Apply to both `ServerConfig` and `ClientConfig`
- [ ] FIPS policy drops ChaCha20 and X25519 (verified)
- [ ] DEFAULT vs FIPS filtering cost benchmark

## Build

```sh
cargo build
cargo test
cargo bench
```

## Run

```sh
cargo run -- --listen 127.0.0.1:8080 --target example.com:443
```

## CI

- **ci.yml** — runs on every push and PR: fmt, clippy, build, test
- **benchmark.yml** — runs on push to `main`: criterion benchmarks stored to `gh-pages` via `benchmark-action/github-action-benchmark`, alerts on >10% regression

## Crates used

| Crate | Purpose |
|---|---|
| `rustls` | Core TLS implementation |
| `rustls-aws-lc-rs` | AWS-LC backed crypto provider (FIPS-capable) |
| `tokio-rustls` | Async tokio integration for rustls |
| `webpki-roots` | Mozilla root CA trust anchors |
| `rcgen` | Self-signed certificate generation |
| `rustls-pki-types` | Shared PKI types across the rustls ecosystem |
| `clap` | CLI argument parsing |
| `tokio` | Async runtime |
| `criterion` | Benchmarking |
