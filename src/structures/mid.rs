use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub password: String,
    pub date_of_birth: u64,
    pub age: u32,
    pub first_name: String,
    pub last_name: String,
    pub alias: String,
    pub email: String,
}
