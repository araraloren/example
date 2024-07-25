use futures_util::{SinkExt, TryStreamExt};
use httpinglib::ReqClient;
use prettytable::Row;
use prettytable::Table;
use reqwest_websocket::Message;
use reqwest_websocket::RequestBuilderExt;

const KEY: &str = "token_20230313000136kwyktxb0tgspm00yo5";

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Httping::parse_env()?;
    let messages = cli.query().await?;
    let mut table = Table::new();

    table.add_row(Row::from_iter(httpinglib::Message::construct_header()));
    if !messages.is_empty() {
        messages
            .iter()
            .map(|msg| msg.construct_row())
            .for_each(|v| {
                table.add_row(Row::from_iter(v));
            });
        table.add_row(Row::from_iter(httpinglib::Message::construct_header()));
        table.printstd();
    }

    Ok(())
}

use cote::prelude::*;

#[derive(Debug, Cote)]
#[cote(aborthelp, width = 100)]
pub struct Httping {
    /// Set the key of request
    #[arg(alias = "-k", value = KEY)]
    key: String,

    /// The target url, for example: www.baidu.com
    #[pos(force = true)]
    host: String,

    /// Enable debug mode
    #[arg(alias = "-d")]
    debug: bool,

    /// Enable verbose mode
    #[arg(alias = "-v")]
    verbose: bool,
}

impl Httping {
    pub async fn query(&self) -> color_eyre::Result<Vec<httpinglib::Message>> {
        let cli = reqwest::ClientBuilder::new().cookie_store(true).build()?;

        if self.debug {
            eprintln!("Try to httping host {}", self.host);
        }

        let reqc = ReqClient::new(cli, &self.key, &self.host).with_debug(self.debug);

        let pingmsg = reqc.req_wssocket_msg("https://www.itdog.cn/http/").await?;

        if self.debug {
            println!("Ping message: {pingmsg}");
        }

        // Creates a GET request, upgrades and sends it.
        let response = reqc
            .cli()
            .get("wss://www.itdog.cn/websockets")
            .upgrade() // Prepares the WebSocket upgrade.
            .send()
            .await?;

        // Turns the response into a WebSocket stream.
        let mut websocket = response.into_websocket().await?;

        // The WebSocket implements `Sink<Message>`.
        websocket.send(Message::Text(pingmsg)).await?;

        let mut messages = vec![];

        if self.debug {
            eprintln!("Waiting for reply..");
        }

        // The WebSocket is also a `TryStream` over `Message`s.
        while let Some(message) = websocket.try_next().await? {
            if self.debug && self.verbose {
                eprintln!("Got message: {message:?}");
            }
            if let Ok(message) = message.json::<httpinglib::Message>() {
                messages.push(message);
            } else if let Message::Text(text) = message {
                if text.contains("\"type\":\"finished\"") {
                    break;
                }
            }
        }
        Ok(messages)
    }
}
