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
    pub accounts: Vec<BankAccount>,
    pub notifications: Vec<Notification>,
}

pub enum AccountType {
    TRADING = 0,
    GIRO = 1,
    SAVING = 2,
    INVEST = 3,
    CREDIT = 4,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct BankAccount {
    pub id: u64,
    pub name: String,
    pub iban: String,
    pub account_type: i32,
    pub balance: i32,
    pub transactions: Vec<Transaction>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub id: u64,
    pub to_name: String,
    pub to_swift: String,
    pub amount: i32,
    pub is_realtime: bool,
    pub execution_date: u64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    pub id: u64,
    pub from: String,
    pub content: String,
    pub date: u64,
}
