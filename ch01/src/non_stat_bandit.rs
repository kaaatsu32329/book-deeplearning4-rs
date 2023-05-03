use rand::Rng;

use crate::bandit::Bandit;

pub trait NonStationaryBandit {
    fn non_stat_play(&mut self, arm_id: usize) -> u64;
}

impl NonStationaryBandit for Bandit {
    fn non_stat_play(&mut self, arm_id: usize) -> u64 {
        let mut rng = rand::thread_rng();
        self.rates[arm_id] += rng.gen_range(-0.1..0.1);
        self.rates[arm_id] = self.rates[arm_id].clamp(0., 1.);

        (self.rates[arm_id] > rng.gen::<f64>()) as u64
    }
}
