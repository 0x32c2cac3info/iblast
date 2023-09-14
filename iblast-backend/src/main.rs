#![allow(unused_imports, unused_variables)]
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
    time::{sleep, Instant}, net::TcpListener,
};

use atty;
use time::macros::offset;
use anyhow::{self, Error, anyhow as ah, bail, Context};
use actix_web::dev::Server;
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
use actix_web::{http::header::ContentType, HttpResponse};
use sqlx::{PgPool, postgres::{PgPoolOptions, PgConnectOptions}};
use url::{Url, ParseError};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
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

    let db_conf = DbSettings::new(&sec::DB_CNX);
    let db = get_cnx_pool(&db_conf).await;
    let web = Web::build(None).await?;        
    let iblast = IBlast::new(web, db);
    Ok(iblast.run_forever().await?)
}



struct Web {
    running: bool,
    port: u16,
    server: Server,
}

impl Web {
    pub async fn build(config: Option<Config>) -> Result<Self, anyhow::Error> {
        let config = config.unwrap_or(Config::default());        
        let address = format!("{}:{}", &config.web.host, &config.web.port);
        let listener = TcpListener::bind(address).await?;
        let it = go(listener);
        it.await
    }
}

async fn get_cnx_pool(db_conf: &DbSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(::std::time::Duration::from_secs(10))
        .connect_lazy_with(db_conf.with_db())
}


struct DbSettings {
    psql: String,
}

impl DbSettings {
    pub fn new(psql: &str) -> Self {
        let psql = String::from(psql);
        Self { psql }
    }
    fn with_db(&self) -> PgConnectOptions {
        use sqlx::ConnectOptions;
        let ops = Url::parse(self.psql.as_str()).unwrap();
        PgConnectOptions::from_url(&ops).unwrap()
    }
} 

async fn go(listener: TcpListener) -> Result<Web, anyhow::Error> {
    let key = Key::try_generate().unwrap();
    // let key = Key::from(sec::SECRET_KEY.as_bytes());
    let msg_store = CookieMessageStore::builder(key).build();
    let framework = FlashMessagesFramework::builder(msg_store).build();
    let port = listener.local_addr().unwrap().port();
    let mut listener = listener.into_std()?;
    listener.set_nonblocking(true)?;
    let server = HttpServer::new(move || {
        App::new()
            .wrap(framework.clone())
            .route("/", web::get().to(nothing))

    })
    .listen(listener)?
    .run();
    let web = Web { running: true, port, server };    
    Ok(web)
}

pub(crate) mod sec {
    pub const SECRET_KEY: &str = "abc12345678900000000000000000000000000";
    
    lazy_static::lazy_static! {
        pub static ref DB_CNX: &'static str = Box::leak(Box::new(
            std::env::var("NEON")
            .unwrap_or("postgres://postgres:password@localhost/db".to_string())
        ));
    }
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

pub async fn nothing() -> HttpResponse {
    info!("Client request incoming");
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body("<!doctype html><html><p>hello 0x5010515 world!<p></html>")
}

struct IBlast {
    web: Web,
    db: PgPool,
}

impl IBlast {
    pub fn new(web: Web, db: PgPool) -> Self {
        Self { web, db }
    }
    async fn run_forever(self) -> Result<(), std::io::Error> {
        self.web.server.await
    }
}
