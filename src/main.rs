use std::net::ToSocketAddrs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_native_tls::native_tls::TlsConnector;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    println!("---------  Request echo `echo.websocket.org`...");
    request("echo.websocket.org").await?;
    println!("---------  Request itdog `www.itdog.cn`...");
    request("www.itdog.cn").await?;

    Ok(())
}

pub async fn request(host: &str) -> color_eyre::Result<()> {
    let addr = &format!("{}:443", host).to_socket_addrs()?.next().unwrap();
    let socket = TcpStream::connect(&addr).await?;
    let connector = TlsConnector::builder().build()?;
    let connector = tokio_native_tls::TlsConnector::from(connector);

    let mut socket = connector.connect(host, socket).await?;

    let data = format!("\
         GET /websockets HTTP/1.0\r\n\
         Host: {}\r\n\
        Upgrade: websocket\r\n\
        Connection: Upgrade\r\n\
        Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
        Sec-WebSocket-Version: 13\r\n\
        Origin: https://{}\r\n\
        User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:128.0) Gecko/20100101 Firefox/128.0\r\n\
         \r\n\
         ", host, host);

    socket.write_all(data.as_bytes()).await?;

    let mut data = Vec::new();
    socket.read_to_end(&mut data).await?;

    println!("{}", String::from_utf8_lossy(&data[..]));

    Ok(())
}
