use clap::Parser;
use std::net::SocketAddr;

use gedeair_backend::{app, cli::Arguments};
use tracing_subscriber::prelude::*;

use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let cli = Arguments::parse();

    let tracing_is_enable = !cli.disable_tracing;
    if tracing_is_enable {
        // Launch tracing
        let filter = tracing_subscriber::filter::Targets::new()
            .with_target("tower_http::trace::on_response", tracing::Level::TRACE)
            .with_target("tower_http::trace::on_request", tracing::Level::TRACE)
            .with_target("tower_http::trace::make_span", tracing::Level::DEBUG)
            .with_default(tracing::Level::INFO);

        let tracing_layer = tracing_subscriber::fmt::layer();

        tracing_subscriber::registry()
            .with(tracing_layer)
            .with(filter)
            .init();
    }

    let app = app().layer(TraceLayer::new_for_http());

    let address: SocketAddr = cli.address.parse().expect(
        &format!(
            "Sorry but address: {} is not correctly formatted",
            cli.address
        )[..],
    );

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Host address is not alvaible");

    axum::serve(listener, app)
        .await
        .expect("Axum server couldn't start");
}
