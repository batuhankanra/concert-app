use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
}
#[derive(Debug,Serialize,Deserialize)]
pub struct Claims{
    pub sub:String,
    pub email:String,
    pub role:String,
    pub exp:usize
}