use plotters::prelude::*;

pub trait Plot {
    fn plot(&self, name: &str) -> Result<(), Box<dyn std::error::Error>>;
}

impl Plot for Vec<f32> {
    fn plot(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut xs = Vec::new();
        let ys = self;
        for i in 0..self.len() {
            xs.push(i as u32);
        }

        let width = 1080;
        let height = 720;
        let path = format!("{}{}{}", "./ch01/graph/", name, ".png");
        let root = BitMapBackend::new(&path, (width, height)).into_drawing_area();

        root.fill(&WHITE)?;

        let (y_min, y_max) = ys
            .iter()
            .fold((f32::NAN, f32::NAN), |(m, n), v| (v.min(m), v.max(n)));

        let font = ("sans-serif", 20);

        let mut chart;

        if y_min < 0. {
            chart = ChartBuilder::on(&root)
                .caption(name, font.into_font())
                .margin(10)
                .x_label_area_size(16)
                .y_label_area_size(42)
                .build_cartesian_2d(*xs.first().unwrap()..*xs.last().unwrap(), y_min..y_max)?;
        } else {
            chart = ChartBuilder::on(&root)
                .caption(name, font.into_font())
                .margin(10)
                .x_label_area_size(16)
                .y_label_area_size(42)
                .build_cartesian_2d(*xs.first().unwrap()..*xs.last().unwrap(), 0f32..y_max)?;
        }

        chart.configure_mesh().draw()?;

        let line_series = LineSeries::new(xs.iter().zip(ys.iter()).map(|(x, y)| (*x, *y)), &RED);
        chart.draw_series(line_series)?;

        Ok(())
    }
}
