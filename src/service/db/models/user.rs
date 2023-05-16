#![allow(dead_code, unused_variables)]

#[derive(Debug, PartialEq, Eq)]
pub struct UserModel {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub fn build_user(name: String, email: String, password: String) -> UserModel {
    UserModel {
        name,
        email,
        password,
    }
}

pub fn get_table_name() -> String {
    "user_auth".into()
}

pub fn create_table_query() -> String {
    let table_name = get_table_name();
    let query = format!("CREATE TABLE IF NOT EXISTS {} (
        id INT NOT NULL AUTO_INCREMENT,
        name VARCHAR(255) NOT NULL,
        email VARCHAR(255) NOT NULL,
        password VARCHAR(255) NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
        PRIMARY KEY (id)
    )", table_name);

    return query;
}

