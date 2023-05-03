use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use egui::plot::Plot;

use ch01::{agent::*, bandit::*};
use utils::ResVecF64;

const RUNS_COUNT: usize = 200;
const BANDIT_SIZE: usize = 10;
const STEP: usize = 1000;
const EPSILON: f64 = 0.1;

fn main() {
    let mut rates = vec![];

    for _ in 0..RUNS_COUNT {
        let bandit = Bandit::new(BANDIT_SIZE);
        let mut agent = Agent::new(EPSILON, 10);

        for _ in 0..STEP {
            let action_target = agent.get_action();
            let reward = bandit.play(action_target);
            agent.update(action_target, reward as f64);
            agent.set_reward_and_log(reward as f64);
        }

        rates.push(agent.get_rates());
    }

    let mut rate_avg = ResVecF64(vec![0.; STEP]);

    for (_, rate) in rates.iter().enumerate() {
        for (j, val) in rate.iter().enumerate() {
            rate_avg.0[j] += val / RUNS_COUNT as f64;
        }
    }

    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "RL in Rust".to_string(),
            resolution: WindowResolution::new(840., 700.),
            ..Default::default()
        }),
        ..Default::default()
    });

    App::new()
        .add_plugins(default_plugins)
        .add_plugin(EguiPlugin)
        .insert_resource(rate_avg)
        .add_system(ui_plot_rates_system)
        .run();
}

fn ui_plot_rates_system(
    mut egui_context: Query<&mut EguiContext, With<PrimaryWindow>>,
    rate_avg: Res<ResVecF64>,
) {
    let Ok(mut ctx) = egui_context.get_single_mut() else { return; };
    egui::Window::new("Rates")
        .fixed_size(egui::Vec2::new(800., 600.))
        .show(ctx.get_mut(), |ui| {
            Plot::new("Rates").data_aspect(500.).show(ui, |plot_ui| {
                plot_ui.line(rate_avg.plot_line().color(egui::Color32::RED).width(1f32))
            });
        });
}
