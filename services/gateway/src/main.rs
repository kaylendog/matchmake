use axum::{
    routing::{get, post},
    Json, Router,
};
use models::Player;
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};

async fn request_queue(Json(player): Json<Player>) -> String {
    player.rank.to_string()
}

async fn hello_world() -> &'static str {
    "kod is bae"
}

// Simple multithreaded GET server

#[tokio::main]
async fn main() {
    // initialise logger
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();

    info!("Initialising server...");

    // create app
    let app = <Router>::new()
        .route("/", get(hello_world))
        .route("/players/queue", post(request_queue));

    // bind and listen
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind socket");
    axum::serve(listener, app)
        .await
        .expect("fatal error encountered")
}
