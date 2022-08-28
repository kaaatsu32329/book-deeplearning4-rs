use rand::Rng;

#[derive(Debug, Clone)]
pub struct Bandit {
    pub rates: Vec<f64>,
}

impl Bandit {
    pub fn new(size: usize) -> Self {
        let rates = vec![rand::thread_rng().gen(); size];
        Bandit { rates }
    }

    pub fn play(&self, arm: u32) -> u32 {
        let rate = self.rates[arm as usize];
        if rate > rand::thread_rng().gen() {
            1
        } else {
            0
        }
    }
}

pub struct Agent {
    epsilon: f64,
    qualitys: Vec<f64>,
    numbers: Vec<u64>,
    size: usize,
}

impl Agent {
    pub fn new(epsilon: f64, size: usize) -> Self {
        let qualitys = vec![0f64; size];
        let numbers = vec![0u64; size];
        Agent {
            epsilon,
            qualitys,
            numbers,
            size,
        }
    }

    pub fn update(&mut self, action: u32, reward: u32) {
        self.numbers[action as usize] += 1;
        self.qualitys[action as usize] +=
            (reward as f64 - self.qualitys[action as usize]) / self.numbers[action as usize] as f64;
    }

    pub fn get_action(&self) -> u32 {
        if self.epsilon > rand::thread_rng().gen() {
            rand::thread_rng().gen_range(1..self.size as u32)
        } else {
            for i in 0..self.size {
                if self.qualitys.iter().fold(f64::NAN, |m, v| v.max(m)) == self.qualitys[i] {
                    return i as u32;
                }
            }
            0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use assert_approx_eq::assert_approx_eq;

    const SIZE: usize = 5;
    const ARM: u32 = 2;
    const EPSILON: f64 = 0.05;
    const ACTION: u32 = 2;
    const REWARD: u32 = 8;

    #[test]
    fn bandit() {
        let bandit = Bandit::new(SIZE);
        let result = bandit.play(ARM);
        assert!(result == 0 || result == 1);
    }

    #[test]
    fn agent() {
        let mut agent = Agent::new(EPSILON, SIZE);
        agent.update(ACTION, REWARD);

        assert_eq!(agent.numbers[ACTION as usize], 1);
        assert_approx_eq!(agent.qualitys[ACTION as usize], REWARD as f64);

        let agent_action = agent.get_action();
        assert!(agent_action < SIZE as u32);
    }
}
