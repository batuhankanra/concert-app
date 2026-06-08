
use sqlx::PgPool;

use crate::{dto::users_dto::{LoginRequest, RegisterRequest}, errors::{AppResult, error::AppError}, model::users::User, utils::{ claims::generate_token, password_hash::{hash_password, verify_password}}};





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



pub async fn find_by_email(db:&PgPool,email:&str)->Result<User,sqlx::Error>{
    sqlx::query_as::<_,User>(
        r#"
        SELECT *
        FROM users
        WHERE email = $1
        "#
    )
    .bind(email)
    .fetch_one(db)
    .await
}


pub async fn login(db:&PgPool,payload:LoginRequest,jwt_secret:&str)->AppResult<String>{
    let user: User =find_by_email(db, &payload.email).await.map_err(|_| AppError::ValidationError("invalid email or password".to_string()))?;


    let valid: bool =verify_password(&payload.password, &user.password);

    if !valid {
        return Err(AppError::InvalidCredentials);
    }

    let token: String =generate_token(user.id.to_string(), user.email, user.role, jwt_secret).map_err(|_| AppError::Internal)?;
    Ok(token)
    
}