use tokio::net::TcpStream;

pub async fn connect_upstream(target: &str) -> Result<TcpStream,  std::io::Error> {
    TcpStream::connect(target).await
}