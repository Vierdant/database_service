use super::db;
use super::db::models::user::build_user;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use mysql::*;
use mysql::prelude::*;

pub enum AuthStatus {
    Authenticated,
    Registered,
    Interrupted,
}

pub fn authenticate(username: String, password: &[u8]) -> Result<AuthStatus> {
    let mut connection = db::get_connection().unwrap();

    let db_password = connection.exec_first::<String, _, _>(
        r"SELECT password FROM user_auth WHERE name = :username", 
        params! {
            "username" => &username,
        }
    )?;

    // user not found
    if db_password.is_none() {
        println!("User not found!");
        return Ok(AuthStatus::Interrupted);
    }

    // verify token
    let db_password = db_password.unwrap();
    let parsed_password = PasswordHash::new(&db_password).unwrap();
    if !Argon2::default().verify_password(password, &parsed_password).is_ok() {
        println!("Password Incorrect!");
        return Ok(AuthStatus::Interrupted);
    }

    println!("Logged in!");

    Ok(AuthStatus::Authenticated)
}

pub fn register(username: String, email: String, password: &[u8], token: &[u8]) -> Result<AuthStatus> {
    let mut connection = db::get_connection().unwrap();
    
    // valid row
    // if email is not "input" that means that registeration row is not used
    let db_email = connection.exec_first::<String, _, _>(
        r"SELECT email FROM user_auth WHERE name = :username", 
        params! {
            "username" => &username,
        }
    )?;

    // user not found
    if db_email.is_none() {
        println!("User not found!");
        return Ok(AuthStatus::Interrupted);
    }

    if db_email.unwrap() != "input" {
        println!("User already registered!");
        return Ok(AuthStatus::Interrupted);
    }
    
    // ensure token
    let db_token = connection.exec_first::<String, _, _>(
        r"SELECT password FROM user_auth WHERE name = :username", 
        params! {
            "username" => &username,
        }
    )?.unwrap();

    // verify token
    let parsed_token = PasswordHash::new(&db_token).unwrap();
    if !Argon2::default().verify_password(token, &parsed_token).is_ok() {
        println!("Token Incorrect!");
        return Ok(AuthStatus::Interrupted);
    }

    println!("Passed!");

    // encrypt password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password(password, &salt).unwrap();
    let stringified_hash = password_hash.to_string();

    let name = username.clone();
    // build model
    let user = build_user(
        username,
        email,
        stringified_hash,
    );

    //update database
    connection.exec_first::<String, _, _>(
        r"UPDATE user_auth SET name = :name, email = :email, password = :password WHERE name = :username", 
        params! {
            "name" => user.name,
            "email" => user.email,
            "password" => user.password,
            "username" => name,
        }
    )?;

    Ok(AuthStatus::Registered)
}