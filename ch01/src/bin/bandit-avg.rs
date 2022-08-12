use ch01::{bandit::*, plot::Plot};

const RUNS: u32 = 200;
const STEPS: u32 = 1000;
const EPSILON: f64 = 0.1;
const SIZE: usize = 10;

fn main() {
    let mut all_rates = Vec::new();

    for _ in 0..RUNS {
        let bandit = Bandit::new(SIZE);
        let mut agent = Agent::new(EPSILON, SIZE);
        let mut total_reward = 0f32;
        let mut total_rewards = Vec::new();
        let mut rates = Vec::new();

        for step in 0..STEPS {
            let action = agent.get_action();
            let reward = bandit.clone().play(action);
            agent.update(action, reward);
            total_reward += reward as f32;

            total_rewards.push(total_reward);
            rates.push(total_reward / (step + 1) as f32);
        }

        println!("{}", total_reward);

        all_rates.push(rates);
    }

    let mut avg_rates = Vec::new();

    for i in 0..all_rates[0].len() {
        let mut element = 0f32;
        for j in 0..200 {
            element += all_rates[j][i];
        }
        element /= 200.;
        avg_rates.push(element);
    }

    avg_rates.plot("average_rate").unwrap();
}
