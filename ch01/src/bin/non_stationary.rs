use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use egui::plot::Plot;

use ch01::{agent::*, alpha_agent::*, bandit::*, non_stat_bandit::*};
use utils::Res2VecF64;

const RUNS_COUNT: usize = 200;
const BANDIT_SIZE: usize = 10;
const STEP: usize = 1000;
const EPSILON: f64 = 0.1;
const ALPHA: f64 = 0.8;

fn main() {
    let mut sample_rates = vec![];
    let mut alpha_rates = vec![];

    let mut bandit = Bandit::new(BANDIT_SIZE);

    for _ in 0..RUNS_COUNT {
        let mut agent = Agent::new(EPSILON, 10);

        for _ in 0..STEP {
            let action_target = agent.get_action();
            let reward = bandit.play(action_target);
            agent.update(action_target, reward as f64);
            agent.set_reward_and_log(reward as f64);
        }

        sample_rates.push(agent.get_rates());
    }

    for _ in 0..RUNS_COUNT {
        let mut alpha_agent = AlphaAgent::new(EPSILON, ALPHA, 10);

        for _ in 0..STEP {
            let action_target = alpha_agent.get_action();
            let reward = bandit.non_stat_play(action_target);
            alpha_agent.update(action_target, reward as f64);
            alpha_agent.set_reward_and_log(reward as f64);
        }

        alpha_rates.push(alpha_agent.get_rates());
    }

    let mut rate_avgs = Res2VecF64(vec![0.; STEP], vec![0.; STEP]);

    for (_, rate) in sample_rates.iter().enumerate() {
        for (j, val) in rate.iter().enumerate() {
            rate_avgs.0[j] += val / RUNS_COUNT as f64;
        }
    }

    for (_, rate) in alpha_rates.iter().enumerate() {
        for (j, val) in rate.iter().enumerate() {
            rate_avgs.1[j] += val / RUNS_COUNT as f64;
        }
    }

    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "RL in Rust".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    });

    App::new()
        .add_plugins(default_plugins)
        .add_plugin(EguiPlugin)
        .insert_resource(rate_avgs)
        .add_system(ui_plot_rates_system)
        .run();
}

fn ui_plot_rates_system(
    mut egui_context: Query<&mut EguiContext, With<PrimaryWindow>>,
    rate_avgs: Res<Res2VecF64>,
) {
    let Ok(mut ctx) = egui_context.get_single_mut() else { return; };
    egui::CentralPanel::default().show(ctx.get_mut(), |ui| {
        Plot::new("Plot").data_aspect(500.).show(ui, |plot_ui| {
            let (sample_plot_line, alpha_plot_line) = rate_avgs.plot_line();
            plot_ui.line(sample_plot_line.color(egui::Color32::WHITE).width(1f32));
            plot_ui.line(alpha_plot_line.color(egui::Color32::RED).width(1f32));
        });
    });
}
