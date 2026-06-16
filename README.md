# tls-proxy

A MITM TLS proxy built to learn [rustls](https://github.com/rustls/rustls) internals from the ground up — and as a proof of concept for integrating rustls with the Fedora system crypto policy ([rustls issue #3056](https://github.com/rustls/rustls/issues/3056)).

The proxy terminates TLS from the client, inspects plaintext HTTP traffic, and re-encrypts to the upstream server. Cipher suites, key exchange groups, and protocol versions are filtered at startup using `/etc/crypto-policies/back-ends/gnutls.config` — the same file GnuTLS reads on Fedora — demonstrating how a future `rustls-platform-provider` crate could respect system-wide crypto policy.

## How it works

```
curl ──TLS──► tls-proxy ──TLS──► github.com
               │     ▲
               │  MITM cert signed by
               │  in-process CA (rcgen)
               ▼
         inspect plaintext
         log method, path, status,
         tls_version, cipher_suite
```

## Run

```sh
cargo run -- --listen 127.0.0.1:8080 --target github.com:443
```

In a second terminal:

```sh
curl -sk https://127.0.0.1:8080/
```

Sample output:

```json
{
  "timestamp": "2026-06-16T08:29:23.553971987+00:00",
  "host": "github.com",
  "status": 301,
  "method": "GET",
  "path": "/",
  "target": "github.com:443",
  "tls_version": "Some(TLSv1_3)",
  "cipher_suite": "Some(TLS13_AES_128_GCM_SHA256)",
  "cert_valid": true,
  "response_time": "507ms"
}
```

The negotiated cipher suite (`TLS13_AES_128_GCM_SHA256`) is one that the Fedora DEFAULT policy permits. If the upstream tried to negotiate something outside the policy, the handshake would fail.

## Fedora crypto policy integration

At startup the proxy reads `/etc/crypto-policies/back-ends/gnutls.config` and maps the INI-style entries to rustls types:

| gnutls.config key | maps to |
|---|---|
| `tls-enabled-cipher = AES-256-GCM` | filters `ALL_CIPHER_SUITES` to those containing `AES_256_GCM` |
| `tls-enabled-group = GROUP-X25519` | filters `ALL_KX_GROUPS` to `X25519` |
| `enabled-version = TLS1.3` | passed to `with_protocol_versions()` |

The result is installed via `CryptoProvider::install_default()` so every TLS connection in the proxy inherits the system policy automatically. On systems without `/etc/crypto-policies/` the proxy falls back to the aws-lc-rs defaults.

## Build

```sh
cargo build
cargo test
cargo bench
```

## Feature checklist

### Phase 1 — TCP plumbing
- [x] CLI: `--listen <addr>` and `--target <addr>` via clap
- [x] Accept TCP connections with tokio `TcpListener`
- [x] Connect to upstream with `TcpStream::connect`
- [x] Forward bytes bidirectionally
- [x] Baseline throughput benchmark

### Phase 2 — TLS client side
- [x] `ClientConfig` with webpki-roots trust anchors
- [x] Wrap upstream in `TlsConnector` via tokio-rustls
- [x] Proxy reaches real HTTPS targets (tested against github.com, example.com)
- [x] Log negotiated cipher suite and TLS version
- [x] Handshake benchmark

### Phase 3 — TLS server side
- [x] Per-host leaf cert signed by in-process CA (rcgen)
- [x] `ServerConfig` built from leaf cert bytes
- [x] `TlsAcceptor` wraps the listener
- [x] `curl --insecure` connects successfully

### Phase 4 — Full MITM bridge
- [x] Decrypt client TLS, re-encrypt to upstream
- [x] Bidirectional TLS bridge with graceful EOF and TLS close_notify handling
- [x] Inspect HTTP method, path, and response status per connection
- [x] JSON log output per connection (serde + chrono)

### Phase 5 — Fedora crypto policy
- [x] Parse `/etc/crypto-policies/back-ends/gnutls.config`
- [x] Map GnuTLS names → rustls cipher suites and kx groups
- [x] Build filtered `CryptoProvider` from active policy
- [x] Install as process-wide default — all TLS inherits system policy
- [x] Protocol versions from policy wired into `ClientConfig`
- [ ] FIPS policy verification (ChaCha20 / X25519 dropped)
- [ ] DEFAULT vs FIPS filtering cost benchmark

## Benchmarks

```
policy_load        7.8 µs    parse gnutls.config from disk (startup, runs once)
to_provider        1.6 µs    build filtered CryptoProvider (startup, runs once)
protocol_versions   62 ns    extract allowed TLS versions (per connection)
inspect_request    166 ns    parse HTTP method + path from request bytes
inspect_response   119 ns    parse status code from first response chunk
```

## CI

GitHub Actions runs on every push and PR:

| Job | Command |
|---|---|
| Format | `cargo fmt --check` |
| Clippy | `cargo clippy -- -D warnings` |
| Test | `cargo test` |
| Bench | `cargo bench --no-run` |

## Crates

| Crate | Purpose |
|---|---|
| `rustls` | Core TLS implementation |
| `tokio-rustls` | Async tokio integration for rustls |
| `webpki-roots` | Mozilla root CA trust anchors |
| `rcgen` | Per-host leaf cert generation |
| `rustls-pki-types` | Shared PKI types across the rustls ecosystem |
| `clap` | CLI argument parsing |
| `tokio` | Async runtime |
| `serde` / `serde_json` | JSON log serialization |
| `chrono` | Timestamps |
| `criterion` | Benchmarking |
