use sqlx::PgPool;

use crate::{dto::users_dto::RegisterRequest, utils::password_hash::{ hash_password}};





pub async fn register(
    db: &PgPool,
    payload: RegisterRequest,
) -> Result<(), anyhow::Error> {

    let password_hash =hash_password(&payload.password).expect("hash problem");

    sqlx::query(
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
    .await?;

    Ok(())
}