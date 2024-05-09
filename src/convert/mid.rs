use crate::structures;
use protobufVsJson::protobuf::data;

use super::ProtobufImpl;

impl ProtobufImpl<structures::mid::User, data::MidUser> for structures::mid::User {
    fn convert_all(all: Vec<structures::mid::User>) -> Vec<data::MidUser> {
        return all
            .iter()
            .map(|x| x.clone().convert_to_protobuf())
            .collect();
    }
    fn convert_to_protobuf(&mut self) -> data::MidUser {
        return data::MidUser {
            id: self.id,
            username: self.username.clone(),
            password: self.password.clone(),
            date_of_birth: self.date_of_birth,
            age: self.age,
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            alias: self.alias.clone(),
            email: self.email.clone(),
        };
    }
}
