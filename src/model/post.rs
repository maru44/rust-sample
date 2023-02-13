use chrono::NaiveDateTime;

#[derive(Describe, Clone, Copy, PartialEq, Eq)]
struct Post {
    id: String,
    user_id: String,
    title: String,
    body: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
