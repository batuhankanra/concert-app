pub mod healt;
pub mod auth_route;
use axum::{Router, http::{Method, header::{AUTHORIZATION, CONTENT_TYPE}}, middleware};
use tower_http::cors::CorsLayer;
use crate::{app_state::AppState, middleware::logger::logger_middleware, };





pub fn create_route(state:AppState)->Router{
    let cors: CorsLayer=CorsLayer::new()
        .allow_methods([Method::GET,Method::POST,Method::PUT,Method::DELETE])
        .allow_origin("http://127.0.0.1:3000".parse::<axum::http::HeaderValue>().unwrap())
        .allow_headers([CONTENT_TYPE,AUTHORIZATION]);
    Router::new()
        .nest("/auth", auth_route::routes())
        .layer(middleware::from_fn(logger_middleware))
        .layer(cors)
        .with_state(state)
}