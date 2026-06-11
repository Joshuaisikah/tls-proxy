use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream as ServerTlsStream;
use tokio_rustls::client::TlsStream as ClientTlsStream;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use crate::inspect::inspect;

pub async fn bridge(client:TcpStream,upstream:TcpStream)->std::io::Result<()>{
    let (mut client_read,mut client_write) = client.into_split();
    let (mut upstream_read,mut upstream_write)= upstream.into_split();
   let(_,_) =  tokio::join!(
        tokio::io::copy(&mut client_read, &mut upstream_write),
        tokio::io::copy(&mut upstream_read, &mut client_write)
    );
    Ok(())
}
pub async fn bridge_tls(client: ServerTlsStream<tokio::net::TcpStream>,upstrean: ClientTlsStream<tokio::net::TcpStream>)->std::io::Result<()>{
    let (mut client_read,mut client_write) = tokio::io::split(client);
    let (mut upstream_read,mut upstream_write)= tokio::io::split(upstrean);
    let mut buf = [0u8; 1024];
    let(_,_) =  tokio::join!(
       async {
           loop {
           let n = client_read.read(&mut buf).await?;
           if n == 0{
               break
           }
           inspect(&buf[..n]);
           upstream_write.write_all(&buf[..n]).await?;
             }
                 Ok::<(),std::io::Error>(())
         },
         tokio::io::copy(&mut upstream_read, &mut client_write)

    );
    Ok(())
}
