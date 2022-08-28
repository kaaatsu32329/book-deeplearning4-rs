use std::collections::HashMap;

fn main() {
    let mut value: HashMap<&str, f64> = HashMap::new();
    value.insert("L1", 0.0);
    value.insert("L2", 0.0);

    let mut new_value = value.clone();

    let mut count = 0;
    let mut delta;

    loop {
        *new_value.entry("L1").or_default() =
            0.5 * (-1. + 0.9 * value["L1"]) + 0.5 * (1. + 0.9 * value["L2"]);
        *new_value.entry("L2").or_default() =
            0.5 * (0. + 0.9 * value["L1"]) + 0.5 * (-1. + 0.9 * value["L2"]);

        delta = (new_value["L1"] - value["L1"]).abs();
        delta = delta.max((new_value["L2"] - value["L2"]).abs());

        value = new_value.clone();

        count += 1;

        if delta < 0.0001 {
            println!("{:?}", value);
            println!("{}", count);
            break;
        }
    }
}
