use bevy::prelude::*;
use bevy_egui::egui;
use egui::plot::{Line, PlotPoints};

#[derive(Debug, Resource)]
pub struct ResVecF64(pub Vec<f64>);

impl ResVecF64 {
    pub fn plot_line(&self) -> Line {
        let mut points = vec![];

        for (step, val) in self.0.iter().enumerate() {
            points.push((step as f64, *val));
        }

        let plot_points: PlotPoints = points.iter().map(|(px, py)| [*px, *py]).collect();

        Line::new(plot_points)
    }
}

#[derive(Debug, Resource)]
pub struct Res2VecF64(pub Vec<f64>, pub Vec<f64>);

impl Res2VecF64 {
    pub fn plot_line(&self) -> (Line, Line) {
        let mut points0 = vec![];
        let mut points1 = vec![];

        for (step, val) in self.0.iter().enumerate() {
            points0.push((step as f64, *val));
        }
        for (step, val) in self.1.iter().enumerate() {
            points1.push((step as f64, *val));
        }

        let plot_points0: PlotPoints = points0.iter().map(|(px, py)| [*px, *py]).collect();
        let plot_points1: PlotPoints = points1.iter().map(|(px, py)| [*px, *py]).collect();

        (Line::new(plot_points0), Line::new(plot_points1))
    }
}
