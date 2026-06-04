use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng,Error}};






pub fn hash_password(
    password:&str
)->Result<String,Error>{
    let salt =SaltString::generate(&mut OsRng);

    let argon = Argon2::default();

    let hash=argon.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(hash)
}