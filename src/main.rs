use dts_dev_challenge_backend::app;
use tracing::{Level, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let app = app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let addr = listener.local_addr().unwrap();
    info!("{:<12} - {addr}\n", "LISTENING");
    axum::serve(listener, app).await.unwrap();
}
