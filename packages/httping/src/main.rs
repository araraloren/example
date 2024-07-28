use std::sync::Arc;

use fastwebsockets::handshake;
use fastwebsockets::FragmentCollector;
use fastwebsockets::Frame;
use httpinglib::ReqClient;
use prettytable::Row;
use prettytable::Table;
use reqwest::header::CONNECTION;
use reqwest::header::HOST;
use reqwest::header::SEC_WEBSOCKET_KEY;
use reqwest::header::SEC_WEBSOCKET_VERSION;
use reqwest::header::UPGRADE;
use rustls::ClientConfig;
use std::future::Future;
use tokio_rustls::TlsConnector;

const KEY: &str = "token_20230313000136kwyktxb0tgspm00yo5";

use cote::prelude::*;
use rustls::RootCertStore;
use tokio::net::TcpStream;

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
        let server_host = "www.itdog.cn";

        if self.debug {
            eprintln!("Try to httping host {}", self.host);
        }

        let reqc = ReqClient::new(cli, &self.key, &self.host).with_debug(self.debug);

        let pingmsg = reqc.req_wssocket_msg("https://www.itdog.cn/http/").await?;

        if self.debug {
            eprintln!("Ping message: {pingmsg}");
        }

        // Prepare a tls connection
        let tcp_stream = TcpStream::connect(&format!("{}:443", server_host)).await?;
        let config = ClientConfig::builder()
            .with_root_certificates(RootCertStore::from_iter(
                webpki_roots::TLS_SERVER_ROOTS.iter().cloned(),
            ))
            .with_no_client_auth();
        let tls_connector = TlsConnector::from(Arc::new(config));
        let server_name =
            tokio_rustls::rustls::pki_types::ServerName::try_from(server_host)?.to_owned();
        let tls_stream = tls_connector.connect(server_name, tcp_stream).await?;

        // Prepare a request
        let request = reqc
            .cli()
            .get("wss://www.itdog.cn/websockets")
            .header(HOST, server_host)
            .header(UPGRADE, "websocket")
            .header(CONNECTION, "upgrade")
            .header(SEC_WEBSOCKET_KEY, fastwebsockets::handshake::generate_key())
            .header(SEC_WEBSOCKET_VERSION, "13")
            .build()?;
        let request: http::Request<reqwest::Body> = request.try_into()?;
        let (parts, _) = request.into_parts();
        let request = http::Request::from_parts(parts, String::default());

        let (websocket, _) = handshake::client(&SpawnExecutor, request, tls_stream).await?;
        let mut websocket = FragmentCollector::new(websocket);

        // The WebSocket implements `Sink<Message>`.
        websocket
            .write_frame(Frame::text(fastwebsockets::Payload::Borrowed(
                pingmsg.as_bytes(),
            )))
            .await?;

        let mut messages: Vec<httpinglib::Message> = vec![];

        if self.debug {
            eprintln!("Waiting for reply..");
        }

        // The WebSocket is also a `TryStream` over `Message`s.
        while let Ok(message) = websocket.read_frame().await {
            match message.opcode {
                fastwebsockets::OpCode::Text => {
                    let text = String::from_utf8(message.payload.to_vec())?;

                    if self.debug && self.verbose {
                        eprintln!("Got message: {}", text);
                    }
                    if text.contains("\"type\":\"finished\"") {
                        break;
                    } else {
                        messages.push(serde_json::from_str(&text)?);
                    }
                }
                fastwebsockets::OpCode::Close => {
                    break;
                }
                _ => {}
            }
        }

        websocket
            .write_frame(Frame::close_raw(vec![].into()))
            .await?;

        if self.debug {
            eprintln!("Received message count = {}", messages.len());
        }
        Ok(messages)
    }
}

struct SpawnExecutor;

impl<Fut> hyper::rt::Executor<Fut> for SpawnExecutor
where
    Fut: Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    fn execute(&self, fut: Fut) {
        tokio::task::spawn(fut);
    }
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    rustls::crypto::ring::default_provider()
        .install_default()
        .unwrap();

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
