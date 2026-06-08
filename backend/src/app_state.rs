use sqlx::PgPool;

use crate::config::env::CONFIG;



#[derive(Clone,Debug)]
pub struct AppState{
    pub postgres_db:PgPool,
    pub config:CONFIG,
}