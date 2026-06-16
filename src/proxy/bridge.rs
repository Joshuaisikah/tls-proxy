use tokio_rustls::server::TlsStream as ServerTlsStream;
use tokio_rustls::client::TlsStream as ClientTlsStream;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use crate::inspect::{inspect, inspect_response};

pub async fn bridge_tls(
    client: ServerTlsStream<tokio::net::TcpStream>,
    upstrean: ClientTlsStream<tokio::net::TcpStream>,
    mut log: crate::report::Report,
    start: std::time::Instant,
) -> std::io::Result<()> {
    let (mut client_read, mut client_write) = tokio::io::split(client);
    let (mut upstream_read, mut upstream_write) = tokio::io::split(upstrean);

    let (req_result, status_result) = tokio::join!(
        async {
            let mut buf = [0u8; 1024];
            let mut method = String::new();
            let mut path = String::new();
            loop {
                let n = match client_read.read(&mut buf).await {
                    Ok(0) | Err(_) => break,
                    Ok(n) => n,
                };
                if let Some((m, p)) = inspect(&buf[..n]) {
                    method = m;
                    path = p;
                }
                if upstream_write.write_all(&buf[..n]).await.is_err() { break; }
            }
            let _ = upstream_write.shutdown().await;
            Ok::<(String, String), std::io::Error>((method, path))
        },
        async {
            let mut buf = [0u8; 1024];
            let mut status = 0u16;
            let mut first = true;
            loop {
                let n = match upstream_read.read(&mut buf).await {
                    Ok(0) | Err(_) => break,
                    Ok(n) => n,
                };
                if first {
                    if let Some(code) = inspect_response(&buf[..n]) {
                        status = code;
                    }
                    first = false;
                }
                if client_write.write_all(&buf[..n]).await.is_err() { break; }
            }
            let _ = client_write.shutdown().await;
            Ok::<u16, std::io::Error>(status)
        }
    );

    if let Ok((method, path)) = req_result {
        log.method = method;
        log.path = path;
    }
    log.status = status_result.unwrap_or(0);
    log.response_time = format!("{}ms", start.elapsed().as_millis());
    log.write_log();
    Ok(())
}
