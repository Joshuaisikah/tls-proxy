mod cert;
mod inspect;
mod policy;
mod proxy;
mod report;
mod tls;

use clap::Parser;
use proxy::bridge::bridge;
use proxy::upstream::connect_upstream;

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
    println!("Listening on {}", args.listen);
    loop{
        let (client_stream, client_addr) = listener.accept().await?;
        println!("Accepted connection from {}", client_addr);
        let target =args.target.clone();
        tokio::spawn(async move {
            match connect_upstream(target.as_str()).await {
                Err(e) => eprintln!("Failed to connect upstream: {}", e),
                Ok(stream) => {
                    if let Err(e) = bridge(client_stream,stream).await{
                        eprintln!("Failed to bridge upstream: {}", e);
                    }
                }
            }
        });
    }
}
