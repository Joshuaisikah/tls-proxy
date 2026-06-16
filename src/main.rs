mod cert;
mod inspect;
mod policy;
mod proxy;
mod report;
mod tls;

use clap::Parser;
use std::sync::Arc;
use cert::CertAuthority;
use proxy::listener::handle_connection;
use policy::fedora::FedoraPolicy;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    listen: String,
    #[arg(long)]
    target: String,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let policy = Arc::new(FedoraPolicy::load("/etc/crypto-policies/back-ends/gnutls.config"));
    policy.to_provider().install_default().expect("failed to install crypto provider");
    let listener = tokio::net::TcpListener::bind(&args.listen).await?;
    let ca = Arc::new(CertAuthority::new()?);
    println!("Listening on {}", args.listen);
    loop {
        let (client_stream, client_addr) = listener.accept().await?;
        println!("Accepted connection from {}", client_addr);
        let target = args.target.clone();
        let ca = ca.clone();
        let policy = policy.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(client_stream, &target, ca, policy).await {
                eprintln!("Connection error: {}", e);
            }
        });
    }
}
