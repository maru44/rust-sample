use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Router, Server,
};
use mvc::controller::user::{create_user_con, get_user_con, list_user_con};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/users", get(list_user_con))
        .route("/users", post(create_user_con))
        .route("/users:id", get(get_user_con));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8001));
    tracing::debug!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
