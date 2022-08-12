use ch01::{bandit::*, plot::Plot};

const STEPS: u32 = 1000;
const EPSILON: f64 = 0.1;
const SIZE: usize = 10;

fn main() {
    let bandit = Bandit::new(SIZE);
    let mut agent = Agent::new(EPSILON, SIZE);

    let mut total_reward = 0f32;
    let mut total_rewards = Vec::new();
    let mut rates = Vec::new();

    for step in 0..STEPS {
        let action = agent.get_action();
        let reward = bandit.play(action);
        agent.update(action, reward);
        total_reward += reward as f32;

        total_rewards.push(total_reward);
        rates.push(total_reward / (step + 1) as f32);
    }

    println!("{}", total_reward);

    // Plot reward change
    total_rewards.plot("total_rewards").unwrap();

    // Plot rate change
    rates.plot("rates").unwrap();
}
