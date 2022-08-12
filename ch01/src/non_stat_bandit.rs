use crate::bandit::*;
use rand::Rng;

pub trait NonStatBandit {
    fn new(size: usize) -> Self;
    fn play(&mut self, arm: u32) -> u32;
}

impl NonStatBandit for Bandit {
    fn new(size: usize) -> Self {
        let mut rates = vec![0f64; size];
        for i in 0..size {
            rates[i] = rand::thread_rng().gen();
        }
        Bandit { rates }
    }

    fn play(&mut self, arm: u32) -> u32 {
        self.rates[arm as usize] += 0.2 * (rand::thread_rng().gen::<f64>() - 0.5);
        let rate = self.rates[arm as usize];
        if rate > rand::thread_rng().gen() {
            1
        } else {
            0
        }
    }
}
