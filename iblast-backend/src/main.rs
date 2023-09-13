#![allow(unused_imports, unused_variables)]
use std::net::TcpListener;
use console_subscriber::ConsoleLayer;
use tracing::{self, error, warn, debug, info, instrument};
use tracing_rolling::{Checker, Daily};
use tracing_subscriber::{prelude::*, EnvFilter};
use tokio::{
    fs::{File, OpenOptions},
    io::{self, AsyncSeekExt, AsyncWriteExt},
    sync::{
        self,
        mpsc::{self, channel, Receiver, Sender},
        oneshot, Mutex,
        watch,
    },
    time::{sleep, Instant},
};
use atty;
//use metrics_tracing_context::{MetricsLayer, TracingContextLayer};
//use metrics_util::layers::Stack;
//use metrics_exporter_prometheus::PrometheusBuilder;
use time::macros::offset;
use anyhow::{self, anyhow as ah, bail, Context};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //let metrics_layer = MetricsLayer::new();

    let console_layer = ConsoleLayer::builder().with_default_env().spawn();

    let console_fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(atty::is(atty::Stream::Stdout))
        .with_target(true);

    let log_fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .with_writer(
            Daily::new("iblast-backend.log", "[year][month][day]", offset!(+8))
                .buffered()
                .build()
                .unwrap(),
        );

    let filter_layer =
        EnvFilter::try_from_default_env().or_else(|_err| EnvFilter::try_new("debug"))?;

    let registry = tracing_subscriber::registry()
        .with(log_fmt_layer)
        .with(console_layer)
        .with(console_fmt_layer)
        .with(filter_layer)
        .init();
    //.with(metrics_layer).init();

    Ok(())
}

use actix_web::dev::Server;

struct Web {
    running: bool,
    port: u16,
    server: Server,
}


impl Web {
    pub async fn build(config: Option<Config>) -> Result<Self, anyhow::Error> {
        let config = config.unwrap_or(Config::default());
        let address = format!("{}:{}", &config.web.host, &config.web.port);
        let listener = TcpListener::bind(address)?;
        // let port = listener.local_addr().unwrap().port();
        let it = go(listener);
        it.await
    }
}

use actix_web::{
    web::{
        self,
        Data,
    },
        App,
        HttpServer,
    
};
use actix_web_flash_messages::FlashMessagesFramework;
use actix_web_flash_messages::{
    storage::CookieMessageStore,
};
use actix_web::cookie::Key;

async fn go(listener: TcpListener) -> Result<Web, anyhow::Error> {
    let key = Key::from(sec::SECRET_KEY.as_bytes());
    let msg_store = CookieMessageStore::builder(key).build();
    let framework = FlashMessagesFramework::builder(msg_store).build();
    let port = listener.local_addr().unwrap().port();
    let it = HttpServer::new(move || {
        App::new()
            .wrap(framework.clone())
            .route("/", web::get().to(nothing))

    })
    .listen(listener)?
    .run();
    let web = Web { running: true, port, server: it };    
    Ok(web)
}

pub(crate) mod sec {
    pub const SECRET_KEY: &str = "abc123456789";
}

struct Config {
    web: WebConfig,
}

impl Default for Config {
    fn default() -> Self {
        let web = WebConfig::default();
        Config {
            web,
        }
    }
}

struct WebConfig {
    host: String,
    port: u16,
}

impl Default for WebConfig {
    fn default() -> Self {
        let host = String::from("127.0.0.1");
        let port = 3000;
        Self { host, port }
    }
}

use actix_web::{http::header::ContentType, HttpResponse};

pub async fn nothing() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body("<!doctype html><html><p>hello 0x5010515 world!<p></html>")
}
