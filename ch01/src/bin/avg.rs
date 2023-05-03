use rand::Rng;

fn main() {
    let mut rng1 = rand::thread_rng();
    let mut rewards = vec![];

    for n in 1..11 {
        let reward = rng1.gen::<f64>();
        rewards.push(reward);
        let estimate_q = rewards.iter().sum::<f64>() / n as f64;
        println!("{:2}: {}", n, estimate_q);
    }

    println!("---------------");

    let mut rng2 = rand::thread_rng();
    let mut estimate_q = 0.;

    for n in 1..11 {
        let reward = rng2.gen::<f64>();
        estimate_q += (reward - estimate_q) / n as f64;
        println!("{:2}: {}", n, estimate_q);
    }
}
