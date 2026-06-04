use std::env;


#[derive(Debug,Clone)]
pub struct CONFIG{
    pub app_name: String,
    pub server_port: u16,
    pub postgres_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
}


impl CONFIG {
    pub fn from_env()->Self{
        Self {
            app_name:get_env("APP_NAME"),
            postgres_url:get_env("DATABASE_URL"),
            jwt_secret:get_env("JWT_SECRET"),
            redis_url:get_env("REDIS_URL"),
            server_port:get_env("SERVER_PORT").parse().expect("SERVER_PORT must be a valid number"),
        }
    }
}


fn get_env(key:&str)->String{
    env::var(key).unwrap_or_else(|_| panic!("{key} is not set"))
}