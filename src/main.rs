use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router, Server,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;

use rust_sample::{
    model::user::{User, UserInput},
    persistence::{
        config::new_pg,
        user::{create_user, list_users},
    },
};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(users))
        .route("/users", post(create_user_ep));

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

async fn users() -> impl IntoResponse {
    let pool = new_pg().await;
    match pool {
        Err(e) => {
            eprintln!("failed to pool pg {}", e);
            return (StatusCode::BAD_REQUEST, Json(json!({"message": "a"})));
        }
        Ok(p) => {
            let users = list_users(&p.pool).await;
            match users {
                Err(e) => {
                    eprintln!("failed to pool pg {}", e);
                    return (StatusCode::BAD_REQUEST, Json(json!({"message": "a"})));
                }
                Ok(us) => {
                    // let users: &[User] = &us;
                    println!("{} users", us.len());
                    // let serialized: String = serde_json::to_string(&users)
                    // let j = Json(users);
                    // (StatusCode::OK, j)
                    // let u = &us[0];
                    // (StatusCode::OK, Json(u))
                    return (StatusCode::BAD_REQUEST, Json(json!({"a": us.len()})));
                    // return Json(us);
                }
            }
        }
    }
}

async fn create_user_ep(Json(payload): Json<UserInput>) -> impl IntoResponse {
    let pool = new_pg().await;
    let input = payload.clone();
    println!("{:?}, {:?}", payload, input);

    match pool {
        Err(e) => {
            eprintln!("failed to pool pg {}", e);
            return StatusCode::BAD_REQUEST;
        }
        Ok(p) => {
            let created = create_user(input, &p.pool).await;
            if let Err(e) = created {
                eprintln!("failed to pool pg {}", e);
                return StatusCode::BAD_REQUEST;
            }
        }
    }

    StatusCode::CREATED
}
