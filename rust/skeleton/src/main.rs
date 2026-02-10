use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello from Rust Axum (Backstage Template) - Managed by Traefik @ superlab.app" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
