use sqlx::PgPool;



#[derive(Clone,Debug)]
pub struct AppState{
    pub postgres_db:PgPool,
}