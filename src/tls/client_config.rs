use std::sync::Arc;
use rustls::{ClientConfig, RootCertStore};
pub fn build_client_config()->Arc<ClientConfig>{
    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    Arc::new(
        ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth()
    )
}
