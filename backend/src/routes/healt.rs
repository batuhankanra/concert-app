use axum::{Router, response::IntoResponse, routing::get};

use crate::app_state::AppState;




pub fn healt()->Router<AppState>{
    Router::new().route("/", get(mm))
}

async  fn  mm()->impl IntoResponse{
    "OK - Sunucu Çalışıyor"
}