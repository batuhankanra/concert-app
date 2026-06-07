use axum::{Json, extract::{State}};
use serde_json::{ json};

use crate::{app_state::AppState, dto::users_dto::RegisterRequest, errors::{AppResult}, extractors::validated_json::ValidatedJson, services::auth_services};





pub async fn register_handlers(
    State(state):State<AppState>,
    ValidatedJson(payload):ValidatedJson<RegisterRequest>
)->AppResult<Json<serde_json::Value>>{
   
     auth_services::register(&state.postgres_db, payload).await?;
     Ok(Json(json!({
        "message": "User created successfully"
    })))
        
}