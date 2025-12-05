use std::env;
use crate::current::current;

mod current;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} <power_kw> <voltage> <power_factor> <efficiency>", args[0]);
    }

    let power_kw: f32 = args[1].parse().expect("Invalid power_kw");
    let voltage: f32 = args[2].parse().expect("Invalid voltage");
    let power_factor: f32 = args[3].parse().expect("Invalid power_factor");
    let efficiency: f32 = args[4].parse().expect("Invalid efficiency");

    let i = current(power_kw, voltage, power_factor, efficiency);
    println!("Current: {:.2} A", i);
}
