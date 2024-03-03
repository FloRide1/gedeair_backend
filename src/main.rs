use axum::routing::get;

#[tokio::main]
async fn main() {
    let app = axum::Router::new().route("/status", get(get_status));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Host address is not alvaible");

    axum::serve(listener, app)
        .await
        .expect("Axum server couldn't start");
}

async fn get_status() -> &'static str {
    "UP"
}
