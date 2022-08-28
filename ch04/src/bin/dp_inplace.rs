use std::collections::HashMap;

fn main() {
    let mut value: HashMap<&str, f64> = HashMap::new();
    value.insert("L1", 0.0);
    value.insert("L2", 0.0);

    let mut count = 0;
    let mut delta;

    loop {
        let t = 0.5 * (-1. + 0.9 * value["L1"]) + 0.5 * (1. + 0.9 * value["L2"]);
        delta = (t - value["L1"]).abs();
        *value.entry("L1").or_default() = t;

        let t = 0.5 * (0. + 0.9 * value["L1"]) + 0.5 * (-1. + 0.9 * value["L2"]);
        delta = delta.max((t - value["L2"]).abs());
        *value.entry("L2").or_default() = t;

        count += 1;

        if delta < 0.0001 {
            println!("{:?}", value);
            println!("{}", count);
            break;
        }
    }
}
