use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_rustls::TlsAcceptor;
use tokio_rustls::TlsConnector;
use pki_types::ServerName;
use crate::cert::CertAuthority;
use crate::tls::client_config::build_client_config;
use crate::tls::server_config::build_server_config;
use crate::proxy::upstream::connect_upstream;
use crate::proxy::bridge::bridge_tls;

pub async fn handle_connection(client:TcpStream,target:&str,ca:Arc<CertAuthority>)->Result<(),Box<dyn std::error::Error>>{
    let hostname =target.split(':').next().ok_or("Invalid target: missing hostname")?;
    let (cert_der,key_der) =ca.sign_for_host(hostname)?;
    let server_config =build_server_config(cert_der,key_der)?;
    let tls_client = TlsAcceptor::from(server_config).accept(client).await?;
    let upstream_tcp = connect_upstream(target).await?;
    let client_config = build_client_config();
    let server_name =ServerName::try_from(hostname.to_string())?;
    let tls_upstream =TlsConnector::from(client_config).connect(server_name, upstream_tcp).await?;
    bridge_tls(tls_client, tls_upstream).await?;
    Ok(())
}
