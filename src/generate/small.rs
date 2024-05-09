use rand::Rng;

use super::random_string;
use super::RandomGeneratorTrait;
use crate::structures::small::SmallStructure;

impl RandomGeneratorTrait<SmallStructure> for SmallStructure {
    fn generate_random_amount(amount: u32) -> Vec<SmallStructure> {
        let mut rand = Vec::new();
        let mut rng = rand::thread_rng();
        for i in 0..amount {
            rand.push(SmallStructure {
                val1: rng.gen(),
                val2: random_string(),
                val3: rng.gen(),
                val4: rng.gen(),
            });
        }
        return rand;
    }
}
