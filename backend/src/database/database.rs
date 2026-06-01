use sqlx::{Error, PgPool, postgres::PgPoolOptions};




pub async  fn postgres_connect(url:&str)->Result<PgPool,Error>{
    PgPoolOptions::new().max_connections(10).connect(url).await
}