# TLS Proxy — Build Log & Targets
Daily: 2 hours morning. Weekends: 4 hours.
Rule: every target ends with working cargo build + git commit.
Answer every "Research:" item in NOTES.md as you go.

## Phase 1 — TCP plumbing (Week 1)
- [ ] Day 1: CLI parses --listen and --target. Research: clap derive
- [ ] Day 2: TcpListener accepts + logs connections. Research: tokio vs std listener
- [ ] Day 3: Connect to upstream target. Research: TcpStream::connect
- [ ] Day 4: Forward bytes client to server. Research: tokio::io::copy
- [ ] Day 5: Forward both directions. Research: copy_bidirectional
- [ ] Weekend: throughput bench baseline — pipe 100MB, time it

## Phase 2 — TLS client side (Week 2)
- [ ] Day 1: Add ClientConfig. Research: ClientConfig::builder
- [ ] Day 2: Wrap upstream in TlsConnector. Research: tokio-rustls
- [ ] Day 3: Proxy reaches https://google.com. Research: ServerName
- [ ] Day 4: Print cipher + TLS version. Research: negotiated_cipher_suite
- [ ] Day 5: Print FIPS status. Research: ClientConfig::fips
- [ ] Weekend: handshake latency bench — 100 handshakes, avg

## Phase 3 — TLS server side (Week 3) [SOUL TEST]
- [ ] Day 1: Self-signed cert with rcgen. Research: rustls cert generator
- [ ] Day 2: Build ServerConfig. Research: ServerConfig::builder
- [ ] Day 3: Wrap listener in TlsAcceptor. Research: TlsAcceptor
- [ ] Day 4: curl --insecure connects. Research: server state machine
- [ ] Day 5: Browser connects + sees cert. Research: SAN, common name
- [ ] Weekend: concurrency bench — max simultaneous TLS connections

## Phase 4 — Full bridge (Week 4)
- [ ] Day 1-2: Decrypt client, re-encrypt to server. Research: record layer
- [ ] Day 3-4: Full bidirectional TLS bridge
- [ ] Day 5: Inspect + log both connections
- [ ] Weekend: end-to-end latency overhead vs direct

## Phase 5 — Fedora policy [THE #3056 WORK] (Month 2)
- [ ] Parse /etc/crypto-policies/back-ends/gnutls.config
- [ ] Map gnutls names to rustls cipher suites / kx groups / sig algs
- [ ] Build filtered CryptoProvider from policy
- [ ] Apply to BOTH ServerConfig and ClientConfig
- [ ] Switch to FIPS policy, confirm ChaCha20/X25519 dropped
- [ ] Weekend: bench DEFAULT vs FIPS filtering cost

## Workbench
- [ ] benches/throughput.rs — bytes/sec
- [ ] benches/handshake.rs — handshake latency
- [ ] criterion auto-compares runs; CI tracks across commits
