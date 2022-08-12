use rand::{self, Rng};

fn main() {
    let mut rng = rand::thread_rng();
    let mut quality = 0f64;

    for n in 1..11 {
        let reward: f64 = rng.gen();
        quality += (reward - quality) / (n as f64);
        println!("Q = {}", quality);
    }
}
