use std::str::FromStr;

use futures_util::StreamExt;
use futures_util::{SinkExt, TryStreamExt};
use reqwest_websocket::RequestBuilderExt;
use rustls::crypto::ring;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    ring::default_provider().install_default().unwrap();

    request_reqw("wss://echo.websocket.org").await?;
    request_reqw("wss://www.itdog.cn/websockets").await?;

    request("wss://echo.websocket.org").await?;
    request("wss://www.itdog.cn/websockets").await?;

    Ok(())
}

pub async fn request(url: &str) -> color_eyre::Result<()> {
    let (ws_stream, _) = connect_async(url).await?;

    println!("TOKIO: -- connected!");

    let (mut writer, mut reader) = ws_stream.split();

    writer
        .send(Message::Text(String::from(
            r#"{"task_id":"202407262337451c6sowxc36q865nb5p","task_token":"bd4c908a626dc75c"}"#,
        )))
        .await?;

    // The WebSocket is also a `TryStream` over `Message`s.
    while let Some(message) = reader.try_next().await? {
        eprintln!("TOKIO: Got message: {message:?}");

        if let Message::Text(text) = &message {
            if text.contains("task_id") || text.contains("finished") {
                drop(reader);
                break;
            }
        } else if let Message::Close(_) = &message {
            break;
        }
    }

    println!("TOKIO: Done!");

    Ok(())
}

pub async fn request_reqw(url: &str) -> color_eyre::Result<()> {
    let req = reqwest::Client::builder()
        .build()?
        .get(url)
        .upgrade()
        .send()
        .await?;
    let web_socket = req.into_websocket().await?;

    println!("REQW: -- connected!");

    let (mut writer, mut reader) = web_socket.split();

    writer
        .send(reqwest_websocket::Message::Text(String::from(
            r#"{"task_id":"202407262337451c6sowxc36q865nb5p","task_token":"bd4c908a626dc75c"}"#,
        )))
        .await?;

    // The WebSocket is also a `TryStream` over `Message`s.
    while let Some(message) = reader.try_next().await? {
        eprintln!("REQW: Got message: {message:?}");

        if let reqwest_websocket::Message::Text(text) = &message {
            if text.contains("task_id") || text.contains("finished") {
                drop(reader);
                break;
            }
        } else if let reqwest_websocket::Message::Close { .. } = &message {
            break;
        }
    }

    println!("REQW: Done!");

    Ok(())
}
