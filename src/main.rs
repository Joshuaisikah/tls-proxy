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

#[derive(Parser)]
struct Args {
    #[arg(long)]
    listen: String,
    #[arg(long)]
    target: String,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse() ;
    let listener = tokio::net::TcpListener::bind(&args.listen).await?;
    let ca = Arc::new(CertAuthority::new()?);
    println!("Listening on {}", args.listen);
    loop{
        let (client_stream, client_addr) = listener.accept().await?;
        println!("Accepted connection from {}", client_addr);
        let target =args.target.clone();
        let ca = ca.clone();
        tokio::spawn(async move{
                if let Err(e)= handle_connection(client_stream,&target,ca).await{
                    eprintln!("Connection error: {}", e);
                }
            });

    }
}
