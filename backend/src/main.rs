use dotenv::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::net::SocketAddr;
use tracing::{info,error,warn,debug};

mod app_state;
mod config;
mod database;
mod handlers;
mod middleware;
mod model;
mod routes;
mod services;
mod utils;

#[tokio::main]
async  fn main() {
    //Log
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();
    info!("App started");


    dotenv().ok();
    let config: config::env::CONFIG=config::env::CONFIG::from_env();


    let postgres_db: sqlx::Pool<sqlx::Postgres>=database::database::postgres_connect(&config.postgres_url).await.expect("postgres connection failed");
    match postgres_db.acquire().await {
        Ok(_)=>println!("Postgres db connected"),
        Err(e)=>{
            eprintln!("Postgres db error: {}",e);
            std::process::exit(1);
        }
    }
    let state: app_state::AppState=app_state::AppState{
        postgres_db,
    };

    let app = routes::create_router().with_state(state);
    let addr: SocketAddr=SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("📡 Sunucu http://{} adresinde çalışıyor...", addr);

    let listener=tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap()

}
