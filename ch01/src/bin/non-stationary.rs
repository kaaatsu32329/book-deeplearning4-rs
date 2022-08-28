use ch01::alpha_agent::AlphaAgent;
use ch01::bandit::{Agent, Bandit};
use ch01::non_stat_bandit::NonStatBandit;
use ch01::plot::Plot;

const RUNS: usize = 200;
const STEPS: u32 = 1000;
const EPSILON: f64 = 0.1;
const SIZE: usize = 10;
const ALPHA: f64 = 0.8;

fn main() {
    let mut result = Vec::new();
    let mut sample_rates = Vec::new();

    for _ in 0..RUNS {
        let mut agent = Agent::new(EPSILON, SIZE);
        let bandit: Bandit = NonStatBandit::new(SIZE);
        let mut total_reward = 0f32;
        let mut rates = Vec::new();

        for step in 0..STEPS {
            let action = agent.get_action();
            let reward = bandit.play(action);
            agent.update(action, reward);
            total_reward += reward as f32;

            rates.push(total_reward / (step + 1) as f32)
        }

        sample_rates.push(rates);
    }

    let mut avg_rates = Vec::new();

    for i in 0..sample_rates[0].len() {
        let mut element = 0.;
        for j in 0..RUNS {
            element += sample_rates[j][i];
        }
        element /= RUNS as f32;
        avg_rates.push(element);
    }

    result.push(avg_rates);

    let mut alpha_rates = Vec::new();

    for _ in 0..RUNS {
        let mut agent = AlphaAgent::new(EPSILON, SIZE, ALPHA);
        let bandit: Bandit = NonStatBandit::new(SIZE);
        let mut total_reward = 0f32;
        let mut rates = Vec::new();

        for step in 0..STEPS {
            let action = agent.get_action();
            let reward = bandit.play(action);
            agent.update(action, reward);
            total_reward += reward as f32;

            rates.push(total_reward / (step + 1) as f32);
        }

        alpha_rates.push(rates);
    }

    let mut avg_rates = Vec::new();

    for i in 0..alpha_rates[0].len() {
        let mut element = 0.;
        for j in 0..RUNS {
            element += alpha_rates[j][i];
        }
        element /= RUNS as f32;
        avg_rates.push(element);
    }

    result.push(avg_rates);

    result[0].plot("sample_average").unwrap();
    result[1].plot("alpha_const_update").unwrap();
}
