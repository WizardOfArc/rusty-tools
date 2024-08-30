use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    meters_per_second: f64,
}

fn meters_per_second_to_miles_per_hour(meters_per_second: f64) -> f64 {
    let centimeters_per_second = meters_per_second * 100.0;
    let centimeters_per_hour = centimeters_per_second * 3600.0;
    let inches_per_hour = centimeters_per_hour / 2.54;
    let feet_per_hour = inches_per_hour / 12.0;
    feet_per_hour / 5280.0
}

fn main() {
    let meters_per_second = Args::parse().meters_per_second;
    let miles_per_hour = meters_per_second_to_miles_per_hour(meters_per_second);
    println!("{} meters per second is {} miles per hour", meters_per_second, miles_per_hour);
}
