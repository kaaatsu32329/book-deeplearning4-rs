use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionSpace {
    UP,
    DONW,
    LEFT,
    RIGHT,
    NEUTRAL,
}

#[derive(Debug, Clone, Copy)]
pub enum GridStatus {
    START { reward: f64 },
    GOAL { reward: f64 },
    NORMAL { reward: f64 },
    WALL,
}

#[derive(Debug, Clone)]
pub struct GridWorld {
    pub action_space: ActionSpace,
    pub reward_map: Vec<Vec<GridStatus>>,
    pub goal_state: (usize, usize),
    pub wall_state: Vec<(usize, usize)>,
    pub start_state: (usize, usize), // State grid
    pub agent_state: (usize, usize), // Current grid
}

impl GridWorld {
    pub fn new(start: (usize, usize), goal: (usize, usize), walls: &[(usize, usize)]) -> Self {
        let action_space = ActionSpace::NEUTRAL;
        let mut reward_map = vec![vec![GridStatus::NORMAL { reward: 0. }; 4]; 3]; // ToDo: variable size.
        let goal_state = goal;
        let mut wall_state = Vec::new();
        let start_state = start;
        let agent_state = start;

        reward_map[start.0][start.1] = GridStatus::START { reward: 0.0 };
        reward_map[goal.0][goal.1] = GridStatus::GOAL { reward: 1.0 };

        for wall in walls {
            reward_map[wall.1][wall.0] = GridStatus::WALL;
            wall_state.push(*wall);
        }

        GridWorld {
            action_space,
            reward_map,
            goal_state,
            wall_state,
            start_state,
            agent_state,
        }
    }

    pub fn default() -> Self {
        let mut thread_rnd = rand::thread_rng();
        let action_space = ActionSpace::NEUTRAL;
        let mut reward_map = vec![vec![GridStatus::NORMAL { reward: 0. }; 4]; 3];
        for i in 0..3 {
            for j in 0..4 {
                reward_map[i][j] = GridStatus::NORMAL {
                    reward: thread_rnd.gen::<f64>(),
                };
            }
        }
        let goal_state = (0, 3);
        let wall_state = vec![(1, 1)];
        let start_state = (2, 0);
        let agent_state = start_state;

        reward_map[start_state.0][start_state.1] = GridStatus::START { reward: 0.0 };
        reward_map[goal_state.0][goal_state.1] = GridStatus::GOAL { reward: 1.0 };
        reward_map[wall_state[0].0][wall_state[0].1] = GridStatus::WALL;

        GridWorld {
            action_space,
            reward_map,
            goal_state,
            wall_state,
            start_state,
            agent_state,
        }
    }

    pub fn height(&self) -> usize {
        self.reward_map.len()
    }

    pub fn width(&self) -> usize {
        self.reward_map[0].len()
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.reward_map.len(), self.reward_map[0].len())
    }

    pub fn actions(&self) -> [ActionSpace; 4] {
        [
            ActionSpace::UP,
            ActionSpace::DONW,
            ActionSpace::LEFT,
            ActionSpace::RIGHT,
        ]
    }

    pub fn next_state(&self, action: ActionSpace) -> (usize, usize) {
        let move_action = match action {
            ActionSpace::UP => (-1, 0),
            ActionSpace::DONW => (1, 0),
            ActionSpace::LEFT => (0, -1),
            ActionSpace::RIGHT => (0, 1),
            _ => (0, 0),
        };

        let next_state = (
            self.agent_state.0 as isize + move_action.0,
            self.agent_state.1 as isize + move_action.1,
        );

        let (nx, ny) = next_state;

        if nx < 0 || nx >= self.height() as isize || ny < 0 || ny >= self.width() as isize {
            self.agent_state
        } else if self.wall_state.iter().any(|state| *state == (nx as usize, ny as usize)) {
            self.agent_state
        } else {
            (next_state.0 as usize, next_state.1 as usize)
        }
    }

    pub fn reward(&self, state: (usize, usize)) -> Option<f64> {
        let result = match self.reward_map[state.0][state.1] {
            GridStatus::START { reward } => reward,
            GridStatus::GOAL { reward } => reward,
            GridStatus::NORMAL { reward } => reward,
            GridStatus::WALL => {
                return None;
            }
        };
        Some(result)
    }

    pub fn reset(&mut self) {
        self.agent_state = self.start_state;
    }

    pub fn step(&mut self, action: ActionSpace) -> Option<(f64, bool)> {
        let state = self.agent_state;
        let next_state = self.next_state(action);
        let result = self.reward(state)?;
        let done = next_state == self.goal_state;

        self.agent_state = next_state;

        Some((result, done))
    }
}
