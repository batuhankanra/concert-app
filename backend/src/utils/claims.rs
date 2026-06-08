use chrono::{Duration,Utc};
use jsonwebtoken::{encode,EncodingKey,Header,errors::Error};
use crate::model::users::Claims;


pub fn generate_token(
    user_id:String,
    email:String,
    role:String,
    secret:&str
)->Result<String,Error>{
    let exp: usize=Utc::now().checked_add_signed(Duration::hours(2)).unwrap().timestamp() as usize;

    let claims: Claims=Claims{
        sub:user_id,
        email,
        role,
        exp
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
}