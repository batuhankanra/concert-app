use dotenv::dotenv;
use std::net::SocketAddr;

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
    


    dotenv().ok();
    let config: config::env::CONFIG=config::env::CONFIG::from_env();
    println!("The server has started working ");

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

    let app = routes::create_route(state);
    let addr: SocketAddr=SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener=tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("server is running on http://{}",addr);
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap()

}
