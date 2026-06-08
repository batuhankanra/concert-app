use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{Error, SaltString, rand_core::OsRng}};






pub fn hash_password(
    password:&str
)->Result<String,Error>{
    let salt: SaltString =SaltString::generate(&mut OsRng);

    let argon: Argon2<'_> = Argon2::default();

    let hash: String=argon.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(hash)
}


pub fn verify_password(password:&str,hash:&str)->bool{
    let parsed_hash= match PasswordHash::new(hash){
        Ok(hash)=>hash,
        Err(_)=>return false,
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash).is_ok()

}