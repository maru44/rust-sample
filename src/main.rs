use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router, Server,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use rust_sample::{
    model::user::{User, UserInput},
    persistence::{config::new_pg, user::create_user},
};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/create", post(create_user_ep));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello World!"
}

async fn create_user_ep(Json(payload): Json<UserInput>) -> impl IntoResponse {
    // let user = User {
    //     id: "awer".to_string(),
    //     username: payload.username,
    //     email: payload.email,
    // };

    let pool = new_pg().await;
    // if let Err(e) = pool {
    //     eprintln!("failed to pool pg {}", e);
    //     return StatusCode::BAD_REQUEST;
    // }

    match pool {
        Err(e) => {
            eprintln!("failed to pool pg {}", e);
            return StatusCode::BAD_REQUEST;
        }
        Ok(p) => {
            let conn = p.pool.acquire().await;
            if let Ok(c) = conn {
                let mut cp = &c;
                create_user(payload, cp).await;
            }
        }
    }

    StatusCode::CREATED
}
