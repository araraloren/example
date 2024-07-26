use std::str::FromStr;

use futures_util::StreamExt;
use futures_util::{SinkExt, TryStreamExt};
use rustls::crypto::ring;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::handshake::client::generate_key;
use tokio_tungstenite::tungstenite::http::header::{
    HOST, SEC_WEBSOCKET_KEY, SEC_WEBSOCKET_VERSION, UPGRADE,
};
use tokio_tungstenite::tungstenite::http::{Request, Uri};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    ring::default_provider().install_default().unwrap();

    request("wss://echo.websocket.org").await?;
    request("wss://www.itdog.cn/websockets").await?;

    Ok(())
}

pub async fn request(url: &str) -> color_eyre::Result<()> {
    // Creates a GET request, upgrades and sends it.
    let url = Uri::from_str(url)?;
    let req = Request::get(&url)
        .header("Connection", "Upgrade")
        .header(HOST, url.host().unwrap())
        .header(UPGRADE, "websocket")
        .header(SEC_WEBSOCKET_VERSION, "13")
        .header(SEC_WEBSOCKET_KEY, generate_key())
        .body(())?;

    println!(" --> {req:?}");

    let (ws_stream, _) = connect_async(req).await?;

    let (mut writer, mut reader) = ws_stream.split();

    writer
        .send(Message::Text(String::from(
            r#"{"task_id":"202407262337451c6sowxc36q865nb5p","task_token":"bd4c908a626dc75c"}"#,
        )))
        .await?;

    // The WebSocket is also a `TryStream` over `Message`s.
    while let Some(message) = reader.try_next().await? {
        eprintln!("Got message: {message:?}");

        if let Message::Text(text) = &message {
            if text.contains("task_id") || text.contains("finished") {
                drop(reader);
                break;
            }
        } else if let Message::Close(_) = &message {
            break;
        }
    }

    println!("Done!");

    Ok(())
}
