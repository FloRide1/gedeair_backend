use clap::Parser;
use migration::MigratorTrait;
use std::{net::SocketAddr, time::Duration};

use gedeair_backend::{app, cli::Arguments};
use tracing_subscriber::prelude::*;

use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // Update environnement variable from .env
    dotenvy::dotenv().ok();
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

    let url: String;
    if let Some(given_url) = cli.database_url {
        url = given_url;
    } else {
        let username = cli.postgres_user;
        let password = cli
            .postgres_password
            .expect("Password should be passed in order to connect to the database");
        let host = cli.postgres_host;
        let port = cli.postgres_port;
        let database = cli.postgres_db;
        url = format!("postgres://{username}:{password}@{host}:{port}/{database}");
    }
    let db = get_database_conn(&url, cli.postgres_schema)
        .await
        .expect("Couldn't connect to the database");

    migration::Migrator::up(&db, None)
        .await
        .expect("Migration couldn't proceed correctly");

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

async fn get_database_conn(
    url: &str,
    default_schema: Option<String>,
) -> Result<sea_orm::prelude::DatabaseConnection, sea_orm::prelude::DbErr> {
    let mut opt = sea_orm::ConnectOptions::new(url);
    opt.max_connections(50)
        .min_connections(3)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));

    if let Some(default_schema) = default_schema {
        opt.set_schema_search_path(default_schema);
    }

    sea_orm::Database::connect(opt).await
}
