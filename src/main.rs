use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use std::{
    env,
    net::{Ipv4Addr, SocketAddr},
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    let app = Router::new().route("/api/HttpExample/", get(root));
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, port));
    tracing::info!("listening on {}", addr);

    println!("Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct Message {
    message: String,
}

async fn root() -> Json<Message> {
    Json(Message {
        message: "👋, 🌍!".to_owned(),
    })
}
