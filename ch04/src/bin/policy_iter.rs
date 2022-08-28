use common::gridworld::*;
use ch04::utils::*;

const GAMMA: f64 = 0.9;
const THRESHOULD: f64 = 0.001;
fn main() {
    let env = GridWorld::default();

    let pi = policy_iter(&env, GAMMA, THRESHOULD);

    println!("{:?}", pi);
}
