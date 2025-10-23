use std::env;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::{self, SqlitePoolOptions}, SqlitePool};

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct NewMessage {
    pub content: String,
}

pub async fn add_message(
    State(state): State<AppState>,
    Json(payload): Json<NewMessage>,
) ->Result<Json<Message>, String> {
    let result = sqlx::query_as!(
        Message,
        r#"INSERT INTO messages (content) VALUES (?) RETURNING id, content"#,
        payload.content
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| format!("DB error: {}", e))?;

    Ok(Json(result))
}

pub async fn list_messages(State(state): State<AppState>) -> Json<Vec<Message>> {
    let rows = sqlx::query!(r#"SELECT id, content FROM messages"#)
        .fetch_all(&state.pool)
        .await
        .unwrap();

    Json(rows.into_iter().map(|r| Message { id: r.id, content: r.content }).collect())
}
