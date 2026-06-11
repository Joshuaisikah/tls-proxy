use std::sync::Arc;
use rustls::ServerConfig;
use pki_types::{CertificateDer,PrivateKeyDer,PrivatePkcs8KeyDer};
pub fn build_server_config(cert_der: Vec<u8>,key_der: Vec<u8>)-> Result<Arc<ServerConfig>,rustls::Error>{
    let cert = CertificateDer::from(cert_der);
    let key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(key_der));
    Ok(Arc::new(
        ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(vec![cert], key)?
    ))
}