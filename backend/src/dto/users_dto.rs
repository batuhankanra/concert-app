use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {

    #[validate(length(
        min = 3,
        message = "usern ame must be at least 3 characters"
    ))]
    pub username: String,

    #[validate(email(
        message = "invalid email address"
    ))]
    pub email: String,

    #[validate(length(
        min = 6,
        message = "password must be at least 6 characters"
    ))]
    pub password: String,
}

#[derive(Debug,Deserialize,Validate)]
pub struct LoginRequest{
    #[validate(email)]
    pub email:String,
    #[validate(length(min=6))]
    pub password:String,
}