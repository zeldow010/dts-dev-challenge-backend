pub async fn spawn_app() -> String {
    let app = dts_dev_challenge_backend::app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap()
    });

    format!("http://localhost:{}", port)
}