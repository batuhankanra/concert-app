
use sqlx::PgPool;

use crate::{dto::users_dto::RegisterRequest, errors::{AppResult, error::AppError}, utils::password_hash::hash_password};





pub async fn register(
    db: &PgPool,
    payload: RegisterRequest,
) -> AppResult<()> {

    let password_hash: String =hash_password(&payload.password).map_err(|_| AppError::Internal)?;

    let result=sqlx::query(
        r#"
        INSERT INTO users
        (
            username,
            email,
            password
        )
        VALUES
        ($1,$2,$3)
        "#
    )
    .bind(payload.username)
    .bind(payload.email)
    .bind(password_hash)
    .execute(db)
    .await;

    match result {
        Ok(_)=>Ok(()),


        Err(sqlx::Error::Database(db_err))=>{
            if db_err.constraint()==Some("users_email_key") {
                return Err(AppError::EmailExists);
            }
            Err(AppError::DbError)
        }
        Err(_)=>Err(AppError::DbError)
    }
}