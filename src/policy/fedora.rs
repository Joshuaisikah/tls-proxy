use std::collections::HashSet;
use std::fs;

pub struct FedoraPolicy {
    pub ciphers: HashSet<String>,
    pub groups: HashSet<String>,
    pub versions:HashSet<String>,
}
impl FedoraPolicy {
    pub fn load(path:&str) ->Self{
        let mut ciphers = HashSet::new();
        let mut groups = HashSet::new();
        let mut versions = HashSet::new();
        if let Ok(content) = fs::read_to_string(path) {
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('[') || line.starts_with('#') {
                    continue;
                }
                 if let Some((key, value)) = line.split_once('=') {
                     match key.trim() {
                         "tls-enabled-cipher" => { ciphers.insert(value.trim().to_string()); }
                         "tls-enabled-group"  => { groups.insert(value.trim().to_string()); }
                         "enabled-version"    => { versions.insert(value.trim().to_string()); }
                         _=>{}
                     }
                 }
            }
        }
        Self {ciphers,groups,versions}
    }

    pub fn protocol_versions(&self) -> Vec<&'static rustls::SupportedProtocolVersion> {
        let mut out = Vec::new();
        if self.versions.contains("TLS1.3") { out.push(&rustls::version::TLS13); }
        if self.versions.contains("TLS1.2") { out.push(&rustls::version::TLS12); }
        if out.is_empty() {
            vec![&rustls::version::TLS13, &rustls::version::TLS12]
        } else {
            out
        }
    }

    pub fn to_provider(&self) -> rustls::crypto::CryptoProvider {
        use rustls::crypto::aws_lc_rs;

        if self.ciphers.is_empty() && self.groups.is_empty() {
            return aws_lc_rs::default_provider();
        }

        let cipher_suites: Vec<_> = aws_lc_rs::ALL_CIPHER_SUITES.iter().copied().filter(|cs| {
            let name = format!("{:?}", cs.suite());
            self.ciphers.iter().any(|c| cipher_matches(&name, c))
        }).collect();

        let kx_groups: Vec<_> = aws_lc_rs::ALL_KX_GROUPS.iter().copied().filter(|g| {
            let name = format!("{:?}", g.name());
            self.groups.iter().any(|gr| group_matches(&name, gr))
        }).collect();

        rustls::crypto::CryptoProvider {
            cipher_suites,
            kx_groups,
            ..aws_lc_rs::default_provider()
        }
    }
}

fn cipher_matches(suite_name: &str, gnutls_name: &str) -> bool {
    match gnutls_name {
        "AES-256-GCM"       => suite_name.contains("AES_256_GCM"),
        "AES-128-GCM"       => suite_name.contains("AES_128_GCM"),
        "AES-256-CBC"       => suite_name.contains("AES_256_CBC"),
        "AES-128-CBC"       => suite_name.contains("AES_128_CBC"),
        "CHACHA20-POLY1305" => suite_name.contains("CHACHA20_POLY1305"),
        _ => false,
    }
}

fn group_matches(group_name: &str, gnutls_name: &str) -> bool {
    match gnutls_name {
        "GROUP-X25519"    => group_name.contains("X25519"),
        "GROUP-SECP256R1" => group_name.contains("secp256r1"),
        "GROUP-SECP384R1" => group_name.contains("secp384r1"),
        "GROUP-SECP521R1" => group_name.contains("secp521r1"),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_missing_file_gives_empty_sets() {
        let policy = FedoraPolicy::load("/nonexistent/path/rustls.config");
        assert!(policy.ciphers.is_empty());
        assert!(policy.groups.is_empty());
        assert!(policy.versions.is_empty());
    }

    #[test]
    fn test_protocol_versions_fallback_when_empty() {
        let policy = FedoraPolicy::load("/nonexistent/path/rustls.config");
        let versions = policy.protocol_versions();
        assert_eq!(versions.len(), 2);
    }

    #[test]
    fn test_to_provider_fallback_when_empty() {
        let policy = FedoraPolicy::load("/nonexistent/path/rustls.config");
        let provider = policy.to_provider();
        assert!(!provider.cipher_suites.is_empty());
        assert!(!provider.kx_groups.is_empty());
    }

    #[test]
    fn test_load_real_policy_file() {
        let policy = FedoraPolicy::load("/etc/crypto-policies/back-ends/gnutls.config");
        assert!(policy.ciphers.contains("AES-256-GCM"));
        assert!(policy.groups.contains("GROUP-X25519"));
        assert!(policy.versions.contains("TLS1.3"));
    }
}