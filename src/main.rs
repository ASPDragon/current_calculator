use clap::Parser;

mod current;
use crate::current::current;

#[derive(Parser, Debug)]
#[command(author, version, about = "Calculates current based on power and voltage")]
struct Args {
    /// Power value (e.g., 100w, 1.5kw)
    #[arg(short, long)]
    power: String,

    /// Voltage in Volts
    #[arg(short, long, default_value_t = 230.0)]
    voltage: f32,

    /// Power factor (cos φ)
    #[arg(short = 'f', long, default_value_t = 1.0)]
    power_factor: f32,

    /// Efficiency (0.0 - 1.0)
    #[arg(short, long, default_value_t = 1.0)]
    efficiency: f32,
}

fn parse_power(s: &str) -> f32 {
    let s = s.to_lowercase();
    if s.ends_with("kw") {
        s.trim_end_matches("kw").parse().expect("Invalid kw")
    } else if s.ends_with("w") {
        s.trim_end_matches("w").parse::<f32>().expect("Invalid w") / 1000.0
    } else {
        s.parse().expect("Invalid power")
    }
}

fn main() {
    let args = Args::parse();

    let p_kw = parse_power(&args.power);
    let i = current(p_kw, args.voltage, args.power_factor, args.efficiency);

    println!("Results for {} (Parsed: {} kW):", args.power, p_kw);
    println!("Current: {:.2} A", i);
}