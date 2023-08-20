use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub roles: HashSet<String>,
    pub verified: bool,
    pub tokens: Vec<UserToken>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct UserToken {
    pub token: String,
    pub ip: String,
    pub created_at: i64,
    pub last_used: i64,
}
