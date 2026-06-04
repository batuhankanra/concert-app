use axum::{Json, extract::State};
use serde_json::{Value, json};

use crate::{app_state::AppState, dto::users_dto::RegisterRequest, services::auth_services};





pub async fn register_handlers(
    State(state):State<AppState>,
    Json(payload):Json<RegisterRequest>
)->Json<Value>{
    match auth_services::register(&state.postgres_db, payload).await {
        Ok(_)=>{
            Json(json!({"msg":"user created"}))
        },
        Err(err)=>Json(json!({"err:":&err.to_string()}))

    }
}