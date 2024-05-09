use rand::{distributions::Alphanumeric, Rng};

pub mod large;
pub mod mid;
pub mod small;

pub trait RandomGeneratorTrait<T> {
    fn generate_random_amount(amount: u32) -> Vec<T>;
}

pub fn random_string() -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    return s;
}
