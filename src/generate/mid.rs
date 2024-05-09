use rand::Rng;

use crate::structures::mid::User;

use super::random_string;
use super::RandomGeneratorTrait;

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
            })
        }
        return rand;
    }
}
