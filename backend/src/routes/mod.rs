use axum::{Router, routing::get};

use crate::{app_state::AppState, routes::healt::health};

pub mod healt;


pub fn create_router()->Router<AppState>{
    Router::new()
        .route("/", get(health))
}