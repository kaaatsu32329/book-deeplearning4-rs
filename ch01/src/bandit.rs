use rand::Rng;

pub struct Bandit {
    pub(crate) rates: Vec<f64>,
}

impl Bandit {
    pub fn new(size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut rates = vec![];

        for _ in 0..size {
            rates.push(rng.gen::<f64>());
        }

        Self { rates }
    }

    pub fn play(&self, arm_id: usize) -> u64 {
        let mut rng = rand::thread_rng();

        (self.rates[arm_id] > rng.gen::<f64>()) as u64
    }
}
