use crate::gridworld::GridWorld;

use piston_window::*;

use crate::gridworld::*;

const WIDTH: u32 = 720;
const HEIGHT: u32 = 480;

pub trait Renderer {
    fn render_value(&mut self);
}

impl Renderer for GridWorld {
    // Issue: Cannot display text.
    // ToDo: Fix above.
    fn render_value(&mut self) {
        let xsize = self.width();
        let ysize = self.height();
        let grid_size = [WIDTH / xsize as u32, HEIGHT / ysize as u32];
        let size = [grid_size[0] * xsize as u32, grid_size[1] * ysize as u32];

        let mut window: PistonWindow = WindowSettings::new("title", size)
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed. Error: {}", e));

        let font = format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/asset/OpenSans-Regular.ttf"
        );

        let mut glyphs = window.load_font(font).expect("No font found!");

        window.set_lazy(true);

        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g, _| {
                clear([0.5, 1.0, 0.5, 1.0], g);

                // Draw grid rectangle and text
                for w in 0..xsize {
                    for h in 0..ysize {
                        let grid_color;
                        let text_color;
                        let reward_text;
                        match self.reward_map[h][w] {
                            GridStatus::START { reward } => {
                                grid_color = [0.1, 0.1, 0.5 + 0.05 * reward as f32, 1.0];
                                reward_text = format!("S{:.2}", reward);
                                text_color = [0.0, 0.0, 0.0, 1.0];
                            }
                            GridStatus::GOAL { reward } => {
                                grid_color = [0.1, 0.5 + 0.05 * reward as f32, 0.1, 1.0];
                                reward_text = format!("G{:.2}", reward);
                                text_color = [0.0, 0.0, 0.0, 1.0];
                            }
                            GridStatus::NORMAL { reward } => {
                                grid_color = [0.5 + 0.5 * reward as f32, 0.0, 0.0, 1.0];
                                reward_text = format!("R{:.2}", reward);
                                text_color = [0.0, 0.0, 0.0, 1.0];
                            }
                            GridStatus::WALL => {
                                grid_color = [0.1, 0.1, 0.1, 1.0];
                                reward_text = String::from("");
                                text_color = [1.0, 1.0, 1.0, 1.0];
                            }
                        }
                        rectangle(
                            grid_color,
                            [
                                (w * grid_size[0] as usize) as f64,
                                (h * grid_size[1] as usize) as f64,
                                ((w + 1) * grid_size[0] as usize) as f64,
                                ((h + 1) * grid_size[1] as usize) as f64,
                            ],
                            c.transform,
                            g,
                        );
                        text::Text::new_color(text_color, 24)
                            .draw(
                                &reward_text,
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(
                                    (w * grid_size[0] as usize) as f64 + (grid_size[0] as f64) / 8.,
                                    ((h + 1) * grid_size[1] as usize) as f64
                                        - (grid_size[1] as f64) / 4.,
                                ),
                                g,
                            )
                            .unwrap();
                        // ToDo: Draw text.
                    }
                }

                // Draw grid line
                for h in 1..ysize {
                    line(
                        color::BLACK,
                        1.0,
                        [
                            0.0,
                            (h * grid_size[1] as usize) as f64,
                            size[0] as f64,
                            (h * grid_size[1] as usize) as f64,
                        ],
                        c.transform,
                        g,
                    );
                }
                for w in 1..xsize {
                    line(
                        color::BLACK,
                        1.0,
                        [
                            (w * grid_size[0] as usize) as f64,
                            0.0,
                            (w * grid_size[0] as usize) as f64,
                            size[1] as f64,
                        ],
                        c.transform,
                        g,
                    );
                }
            });
        }
    }
}
