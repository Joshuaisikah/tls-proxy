use std::sync::Arc;
use rustls::{ClientConfig, RootCertStore, SupportedProtocolVersion};

pub fn build_client_config(versions: &[&'static SupportedProtocolVersion]) -> Result<Arc<ClientConfig>, rustls::Error> {
    let provider = rustls::crypto::CryptoProvider::get_default()
        .expect("provider must be installed before building config")
        .clone();
    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    Ok(Arc::new(
        ClientConfig::builder_with_provider(provider)
            .with_protocol_versions(versions)?
            .with_root_certificates(root_store)
            .with_no_client_auth()
    ))
}
