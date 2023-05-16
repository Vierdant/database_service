use mysql::{prelude::*, PooledConn};

pub mod user;

// user table
pub fn ensure_user_model(mut connection: PooledConn) {
    let query = user::create_table_query();
    connection.query_drop(query).unwrap();
}