use webserver::serve_all_applications;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, serve_all_applications())
        .await
        .unwrap();
}
