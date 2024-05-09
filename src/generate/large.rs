use rand::Rng;

use crate::structures::large::{BankAccount, Notification, Transaction, User};

use super::{random_string, RandomGeneratorTrait};

impl RandomGeneratorTrait<User> for User {
    fn generate_random_amount(amount: u32) -> Vec<User> {
        let mut rand = Vec::new();
        let mut rng = rand::thread_rng();
        for i in 0..amount {
            rand.push(User {
                id: rng.gen(),
                username: random_string(),
                password: random_string(),
                date_of_birth: rng.gen(),
                age: rng.gen(),
                first_name: random_string(),
                last_name: random_string(),
                alias: random_string(),
                email: random_string(),
                accounts: BankAccount::generate_random_amount(5),
                notifications: Notification::generate_random_amount(100),
            });
        }
        return rand;
    }
}

impl RandomGeneratorTrait<BankAccount> for BankAccount {
    fn generate_random_amount(amount: u32) -> Vec<BankAccount> {
        let mut rand = Vec::new();
        let mut rng = rand::thread_rng();
        for i in 0..amount {
            rand.push(BankAccount {
                id: rng.gen(),
                name: random_string(),
                iban: random_string(),
                account_type: 1,
                balance: rng.gen(),
                transactions: Transaction::generate_random_amount(100),
            })
        }
        return rand;
    }
}

impl RandomGeneratorTrait<Transaction> for Transaction {
    fn generate_random_amount(amount: u32) -> Vec<Transaction> {
        let mut rand = Vec::new();
        let mut rng = rand::thread_rng();
        for i in 0..amount {
            rand.push(Transaction {
                id: rng.gen(),
                to_name: random_string(),
                to_swift: random_string(),
                amount: rng.gen(),
                is_realtime: rng.gen(),
                execution_date: rng.gen(),
            });
        }
        return rand;
    }
}

impl RandomGeneratorTrait<Notification> for Notification {
    fn generate_random_amount(amount: u32) -> Vec<Notification> {
        let mut rand = Vec::new();
        let mut rng = rand::thread_rng();
        for i in 0..amount {
            rand.push(Notification {
                id: rng.gen(),
                from: random_string(),
                content: random_string(),
                date: rng.gen(),
            })
        }
        return rand;
    }
}
