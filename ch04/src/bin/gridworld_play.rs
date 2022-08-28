use common::{gridworld::*, gridworld_render::*};
use rand::{self, Rng};

fn main() {
    let mut grid_env = GridWorld::new((2, 0), (0, 3), &[(1, 1)]);

    for h in 0..grid_env.height() {
        for w in 0..grid_env.width() {
            if let GridStatus::NORMAL { reward: _ } = grid_env.reward_map[h][w] {
                grid_env.reward_map[h][w] = GridStatus::NORMAL {
                    reward: rand::thread_rng().gen_range(0.0..1.0),
                };
            }
        }
    }

    grid_env.render_value();
}
