use chrono::DateTime;

pub struct User {
    pub id: u64,
    pub username: String,
    pub password: String,
    pub date_of_birth: u64,
    pub age: u8,
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

pub struct BankAccount {
    pub id: u64,
    pub name: String,
    pub iban: String,
    pub account_type: AccountType,
    pub balance: i32,
    pub transactions: Vec<Transaction>,
}

pub struct Transaction {
    pub id: u64,
    pub to_name: String,
    pub to_swift: String,
    pub amount: i32,
    pub is_realtime: bool,
    pub execution_date: DateTime,
}

pub struct Notification {
    pub id: u64,
    pub from: String,
    pub content: String,
    pub date: DateTime,
}
