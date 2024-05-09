use crate::structures::large;
use protobufVsJson::protobuf::data;

use super::ProtobufImpl;

impl ProtobufImpl<large::User, data::LargeUser> for large::User {
    fn convert_all(all: Vec<large::User>) -> Vec<data::LargeUser> {
        return all
            .iter()
            .map(|x| x.clone().convert_to_protobuf())
            .collect();
    }
    fn convert_to_protobuf(&mut self) -> data::LargeUser {
        let converted_accounts: Vec<data::large_user::BankAccount> = self
            .accounts
            .iter()
            .map(|x| x.clone().convert_to_protobuf())
            .collect();
        let mut accounts = prost::alloc::vec::Vec::new();
        for account in converted_accounts {
            accounts.push(account);
        }
        let converted_notifications: Vec<data::large_user::Notification> = self
            .notifications
            .iter()
            .map(|x| x.clone().convert_to_protobuf())
            .collect();
        let mut notifications = prost::alloc::vec::Vec::new();
        for notification in converted_notifications {
            notifications.push(notification);
        }
        return data::LargeUser {
            id: self.id,
            username: self.username.clone(),
            password: self.password.clone(),
            date_of_birth: self.date_of_birth,
            age: self.age,
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            alias: self.alias.clone(),
            email: self.email.clone(),
            accounts,
            notifications,
        };
    }
}

impl ProtobufImpl<large::BankAccount, data::large_user::BankAccount> for large::BankAccount {
    fn convert_all(all: Vec<large::BankAccount>) -> Vec<data::large_user::BankAccount> {
        return all
            .iter()
            .map(|x| x.clone().convert_to_protobuf())
            .collect();
    }

    fn convert_to_protobuf(&mut self) -> data::large_user::BankAccount {
        let converted_transactions: Vec<data::large_user::bank_account::Transaction> = self
            .transactions
            .iter()
            .map(|x| x.clone().convert_to_protobuf())
            .collect();
        let mut transactions = prost::alloc::vec::Vec::new();
        for transaction in converted_transactions {
            transactions.push(transaction);
        }

        return data::large_user::BankAccount {
            id: self.id,
            name: self.name.clone(),
            iban: self.iban.clone(),
            account_type: self.account_type,
            balance: self.balance,
            transactions,
        };
    }
}

impl ProtobufImpl<large::Transaction, data::large_user::bank_account::Transaction>
    for large::Transaction
{
    fn convert_all(
        all: Vec<large::Transaction>,
    ) -> Vec<data::large_user::bank_account::Transaction> {
        return all
            .iter()
            .map(|x| x.clone().convert_to_protobuf())
            .collect();
    }

    fn convert_to_protobuf(&mut self) -> data::large_user::bank_account::Transaction {
        return data::large_user::bank_account::Transaction {
            id: self.id,
            to_name: self.to_name.clone(),
            to_swift: self.to_swift.clone(),
            amount: self.amount,
            is_realtime: self.is_realtime,
            execution_date: self.execution_date,
        };
    }
}

impl ProtobufImpl<large::Notification, data::large_user::Notification> for large::Notification {
    fn convert_all(all: Vec<large::Notification>) -> Vec<data::large_user::Notification> {
        return all
            .iter()
            .map(|x| x.clone().convert_to_protobuf())
            .collect();
    }

    fn convert_to_protobuf(&mut self) -> data::large_user::Notification {
        return data::large_user::Notification {
            id: self.id,
            from: self.from.clone(),
            content: self.content.clone(),
            date: self.date,
        };
    }
}
