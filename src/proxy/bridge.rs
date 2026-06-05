use tokio::net::TcpStream;
pub async fn bridge(client:TcpStream,upstream:TcpStream)->std::io::Result<()>{
    let (mut client_read,mut client_write) = client.into_split();
    let (mut upstream_read,mut upstream_write)= upstream.into_split();
   let(_,_) =  tokio::join!(
        tokio::io::copy(&mut client_read, &mut upstream_write),
        tokio::io::copy(&mut upstream_read, &mut client_write)
    );
    Ok(())
}
