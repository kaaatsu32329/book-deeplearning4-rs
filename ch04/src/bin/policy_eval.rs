use common::gridworld::{ActionSpace, GridWorld};
use common::gridworld_render::Renderer;
use std::collections::HashMap;

use ch04::utils::*;

const GAMMA: f64 = 0.9;
const THRESHOULD: f64 = 0.001;

fn main() {
    let mut env = GridWorld::default();
    let pi = HashMap::from([
        (ActionSpace::UP, 0.25),
        (ActionSpace::DONW, 0.25),
        (ActionSpace::LEFT, 0.25),
        (ActionSpace::RIGHT, 0.25),
    ]);
    let mut value = vec![];

    policy_eval(&pi, &mut value, &env, GAMMA, THRESHOULD);
    println!("{:?}", value);

    env.render_value();
}
