use serde::Deserialize;
use validator::Validate;



#[derive(Debug,Deserialize,Validate)]
pub struct RegisterRequest{
    #[validate(length(min=2))]
    pub username :String,
    #[validate(email)]
    pub email:String,
    #[validate(length(min=6))]
    pub password:String
}


