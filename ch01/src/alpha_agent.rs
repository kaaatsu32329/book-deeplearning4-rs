use bevy::prelude::*;
use bevy_egui::egui;
use egui::plot::{Line, PlotPoints};
use rand::Rng;

#[derive(Debug, Resource)]
pub struct AlphaAgent {
    epsilon: f64,
    estimeta_q_s: Vec<f64>,
    alpha: f64,
    total_reward: f64,
    reward_log: Vec<f64>,
    rates: Vec<f64>,
    step_count: usize,
}

impl AlphaAgent {
    pub fn new(epsilon: f64, alpha: f64, action_size: usize) -> Self {
        Self {
            epsilon,
            estimeta_q_s: vec![0.; action_size],
            alpha,
            total_reward: 0.,
            reward_log: vec![],
            rates: vec![],
            step_count: 0,
        }
    }

    pub fn update(&mut self, action_id: usize, reward: f64) {
        self.estimeta_q_s[action_id] += (reward - self.estimeta_q_s[action_id]) * self.alpha;
    }

    pub fn get_action(&self) -> usize {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(self.epsilon) {
            rng.gen_range(0..self.estimeta_q_s.len())
        } else {
            self.estimeta_q_s.len() - 1
        }
    }

    pub fn set_reward_and_log(&mut self, reward: f64) {
        self.total_reward += reward;
        self.reward_log.push(self.total_reward);
        self.rates
            .push(self.total_reward / (self.step_count + 1) as f64);
        self.step_count += 1;
    }

    pub fn get_reward(&self) -> f64 {
        self.total_reward
    }

    pub fn get_rates(&self) -> Vec<f64> {
        self.rates.clone()
    }

    pub fn plot_reward(&self) -> Line {
        let mut points = vec![];

        for (step, reward) in self.reward_log.iter().enumerate() {
            points.push((step as f64, *reward));
        }

        let plot_points: PlotPoints = points.iter().map(|(px, py)| [*px, *py]).collect();

        Line::new(plot_points)
    }

    pub fn plot_rate(&self) -> Line {
        let mut points = vec![];

        for (step, rate) in self.rates.iter().enumerate() {
            points.push((step as f64, *rate));
        }

        let plot_points: PlotPoints = points.iter().map(|(px, py)| [*px, *py]).collect();

        Line::new(plot_points)
    }
}
