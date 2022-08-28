use rand::Rng;

#[derive(Debug, Clone)]
pub struct AlphaAgent {
    epsilon: f64,
    qualitys: Vec<f64>,
    alpha: f64,
    size: usize,
}

impl AlphaAgent {
    pub fn new(epsilon: f64, size: usize, alpha: f64) -> Self {
        let qualitys = vec![0f64; size];
        AlphaAgent {
            epsilon,
            qualitys,
            alpha,
            size,
        }
    }

    pub fn update(&mut self, action: u32, reward: u32) {
        self.qualitys[action as usize] +=
            (reward as f64 - self.qualitys[action as usize]) * self.alpha;
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
    const EPSILON: f64 = 0.05;
    const ACTION: u32 = 2;
    const REWARD: u32 = 8;
    const ALPHA: f64 = 0.2;

    #[test]
    fn alpha_agent() {
        let mut alpha_agent = AlphaAgent::new(EPSILON, SIZE, ALPHA);
        alpha_agent.update(ACTION, REWARD);

        assert_approx_eq!(alpha_agent.qualitys[ACTION as usize], ALPHA * REWARD as f64);

        let agent_action = alpha_agent.get_action();
        assert!(agent_action < SIZE as u32);
    }
}
