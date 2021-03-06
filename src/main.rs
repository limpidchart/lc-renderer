#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_json;

use crate::proto::render::chart_renderer_server::ChartRendererServer;
use crate::renderer::RendererServer;
use slog::{Drain, FnValue, PushFnValue, Record};
use std::net::SocketAddr;
use tokio::signal;
use tokio::sync::oneshot::{self, Receiver, Sender};
use tonic::transport::Server;

mod bar;
mod color;
mod error;
mod margin;
mod point;
mod proto;
mod renderer;
mod scale;
mod size;
mod value;
mod view;

const ENV_LC_RENDERER_ADDR: &str = "LC_RENDERER_ADDR";

pub fn signal_channel() -> (Sender<()>, Receiver<()>) {
    oneshot::channel()
}

pub async fn wait_for_signal(log: slog::Logger, tx: oneshot::Sender<()>) {
    let _ = signal::ctrl_c().await;
    info!(log, "Got signal, exiting");
    let _ = tx.send(());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prepare logger.
    let drain = slog_json::Json::new(std::io::stdout())
        .set_pretty(false)
        .add_key_value(o!(
        "ts" => PushFnValue(move |_ : &Record, ser| {
            ser.emit(chrono::Utc::now().to_rfc3339())
        }),
        "level" => FnValue(move |rinfo : &Record| {
            rinfo.level().as_short_str()
        }),
        "msg" => PushFnValue(move |record : &Record, ser| {
            ser.emit(record.msg())
        }),
        ))
        .build()
        .fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION")));

    // Catch signals.
    let (signal_tx, singal_rx) = signal_channel();
    let _ = tokio::spawn(wait_for_signal(log.clone(), signal_tx));

    // Configure server address from env.
    let addr = std::env::var(ENV_LC_RENDERER_ADDR).expect(&*format!(
        "unable to read {} env variable",
        ENV_LC_RENDERER_ADDR
    ));
    let socket_addr: SocketAddr = addr
        .parse()
        .expect(&*format!("unable to use {} as socket address", addr));

    // Prepare health reporter service.
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<ChartRendererServer<RendererServer>>()
        .await;

    // Start GRPC server.
    info!(log, "Server is started"; "addr" => addr);
    let renderer_server = RendererServer::new(log);
    Server::builder()
        .add_service(health_service)
        .add_service(ChartRendererServer::new(renderer_server))
        .serve_with_shutdown(socket_addr, async {
            singal_rx.await.ok();
        })
        .await?;

    Ok(())
}
