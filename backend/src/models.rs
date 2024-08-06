use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
}

#[derive(Clone)]
pub struct AppState {
    pub users: std::sync::Arc<Mutex<Vec<User>>>,
}
