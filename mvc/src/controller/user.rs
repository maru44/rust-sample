use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use chrono::NaiveDateTime;
use serde::Serialize;
use serde_json::json;

use crate::model::user::{create_user, get_user, list_users, UserInput, UserRow};

#[derive(Serialize)]
struct User {
    id: uuid::Uuid,
    username: String,
    email: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl User {
    fn from(r: &UserRow) -> User {
        User {
            id: uuid::Uuid::from_u128_le(r.id.to_u128_le()),
            username: r.username.to_string(),
            email: r.email.to_string(),
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

pub async fn create_user_con(Json(input): Json<UserInput>) -> impl IntoResponse {
    let creation = create_user(input).await;

    match creation {
        Err(e) => {
            eprintln!("fialed to create user {}", e);
            StatusCode::BAD_REQUEST
        }
        Ok(_) => StatusCode::CREATED,
    }
}

pub async fn get_user_con(Path(id): Path<uuid::Uuid>) -> impl IntoResponse {
    let get = get_user(id).await;

    match get {
        Err(e) => {
            eprintln!("fialed to create user {}", e);
            (StatusCode::BAD_REQUEST, Json(json!({})))
        }
        Ok(u) => match u {
            Some(u) => (StatusCode::OK, Json(json!(User::from(&u)))),
            None => (StatusCode::NOT_FOUND, Json(json!({ "id": id }))),
        },
    }
}

pub async fn list_user_con() -> impl IntoResponse {
    let list = list_users().await;

    match list {
        Err(e) => {
            eprintln!("fialed to create user {}", e);
            (StatusCode::BAD_REQUEST, Json(json!({})))
        }
        Ok(us) => {
            let users = us.iter().map(User::from).collect::<Vec<User>>();
            return (StatusCode::OK, Json(json!(users)));
        }
    }
}
