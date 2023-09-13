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
    time::{sleep, Instant},
};
use atty;
//use metrics_tracing_context::{MetricsLayer, TracingContextLayer};
//use metrics_util::layers::Stack;
//use metrics_exporter_prometheus::PrometheusBuilder;
use time::macros::offset;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Build our tower layers
    // Note that the `Layer` abstraction is to resemble
    // the nesting of futures, and were named layers
    // to resemble the onion's layering structure.
    //
    // Note: If ogres are like onions because they have
    // layers, and tower::Layer is like an onion, then
    // ogres like Shrek must be tower::Layer's too.)
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
