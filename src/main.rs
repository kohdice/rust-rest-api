use std::env;

use axum::{Router, response::Json, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a valid number");

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Json<&'static str> {
    Json("Hello, World!")
}
