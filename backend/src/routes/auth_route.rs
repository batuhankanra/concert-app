use axum::{Router, routing::post};

use crate::{app_state::AppState, handlers::auth_hadnlers};





pub fn routes()->Router<AppState>{
    Router::new()
    .route("/register", post(auth_hadnlers::register_handlers))
    .route("/login", post(auth_hadnlers::login_handlers))
}