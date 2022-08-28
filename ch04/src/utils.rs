use common::gridworld::{ActionSpace, GridWorld};
use std::{collections::HashMap, vec};
use ordered_float::*;

fn eval_onestep(
    pi: &HashMap<ActionSpace, f64>,
    value: &mut Vec<Vec<f64>>,
    env: &GridWorld,
    gamma: f64,
) {
    for h in 0..env.height() {
        for w in 0..env.width() {
            if (h, w) == env.goal_state {
                continue;
            } else if env.wall_state.iter().any(|state| *state == (h, w)) {
                value[h][w] = -f64::INFINITY;
            } else {
                let mut new_v = 0.;
                for action in pi.keys() {
                    let (next_w, next_h) = env.next_state(*action);
                    let reward = match env.reward((h, w)) {
                        Some(r) => r,
                        None => 0.,
                    };
                    new_v += pi[action] * (reward + gamma * value[next_h][next_w]);
                }
                value[h][w] = new_v;
            }
        }
    }
}

pub fn policy_eval(
    pi: &HashMap<ActionSpace, f64>,
    value: &mut Vec<Vec<f64>>,
    env: &GridWorld,
    gamma: f64,
    threshould: f64,
) {
    value.resize_with(env.reward_map.len(), Default::default);
    for element in value.iter_mut() {
        element.resize_with(env.reward_map[0].len(), Default::default);
    }
    loop {
        let latest_value = value.clone();
        eval_onestep(pi, value, env, gamma);

        let mut delta = 0.;

        for h in 0..env.height() {
            for w in 0..env.width() {
                let t = (value[h][w] - latest_value[h][w]).abs();
                if delta < t {
                    delta = t;
                }
            }
        }
        if delta < threshould {
            break;
        }
    }
}

fn greedy_policy(value: &mut Vec<Vec<f64>>, env: &GridWorld, gamma: f64) -> HashMap<ActionSpace, f64> {
    value.resize_with(env.reward_map.len(), Default::default);
    for element in value.iter_mut() {
        element.resize_with(env.reward_map[0].len(), Default::default);
    }
    let mut pi = HashMap::from([
        (ActionSpace::UP, 0.),
        (ActionSpace::DONW, 0.),
        (ActionSpace::LEFT, 0.),
        (ActionSpace::RIGHT, 0.),
    ]);
    for _ in 0..env.height() {
        for _ in 0..env.width() {
            let mut action_values: HashMap<ActionSpace, f64> = HashMap::default();
            for action in env.actions() {
                let next_state = env.next_state(action);
                let reward = match env.reward(next_state) {
                    Some(r) => r,
                    None => 0.,
                };
                action_values
                    .entry(action)
                    .or_insert(reward + gamma * value[next_state.1][next_state.0]);
            }
            let max_action = action_values.iter().max_by_key(|entry| OrderedFloat(*entry.1)).unwrap();
            pi.insert(*max_action.0, 1.0);
        }
    }
    pi
}

pub fn policy_iter(
    env: &GridWorld,
    gamma: f64,
    threshould: f64,
) -> HashMap<ActionSpace, f64> {
    let mut pi: HashMap<ActionSpace, f64> = Default::default();
    let mut value: Vec<Vec<f64>> = vec![];
    loop {
        policy_eval(&pi, &mut value, env, gamma, threshould);
        let new_pi = greedy_policy(&mut value, env, gamma);

        if pi == new_pi {
            break;
        }
        pi = new_pi;
    }
    pi
}

fn value_iter_onestep(
    value: &mut Vec<Vec<f64>>,
    env: &GridWorld,
    gamma: f64,
) {
    for h in 0..env.height() {
        for w in 0..env.width() {
            let mut action_value = vec![];
            if (h, w) != env.goal_state {
                for action in env.actions() {
                    let next_state = env.next_state(action);
                    let reward = match env.reward(next_state) {
                        Some(r) => r,
                        None => 0.
                    };
                    action_value.push(reward + gamma * value[next_state.0][next_state.1]);
                }
                value[h][w] = action_value.iter().fold(f64::NAN, |m, v| v.max(m));
            }
        }
    }
}

pub fn value_iter(
    value: &mut Vec<Vec<f64>>,
    env: &GridWorld,
    gamma: f64,
    threshould: f64,
) {
    value.resize_with(env.reward_map.len(), Default::default);
    for element in value.iter_mut() {
        element.resize_with(env.reward_map[0].len(), Default::default);
    }
    loop {
        let latest_value = value.clone();
        value_iter_onestep(value, env, gamma);

        let mut delta = 0.;

        for h in 0..env.height() {
            for w in 0..env.width() {
                let t = (value[h][w] - latest_value[h][w]).abs();
                if delta < t {
                    delta = t;
                }
            }
        }
        if delta < threshould {
            break;
        }
    }
}
