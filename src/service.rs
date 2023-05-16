mod db;
mod auth;

pub fn initiate_db() {
    db::ensure_models();
}