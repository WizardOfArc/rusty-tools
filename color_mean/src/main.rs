use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  color_1: String,
  color_2: String,
}

fn mean_hex(hex_1: &str, hex_2: &str) ->  String {
    let num1 = u16::from_str_radix(hex_1, 16).unwrap();
    let num2 = u16::from_str_radix(hex_2, 16).unwrap();
    let mean = (num1 + num2) / 2;
    format!("{:x}", mean)
}

fn find_mean_color(color_1: &str, color_2: &str) ->  String {
    let first_red = &color_1[..2];
    let first_green= &color_1[2..4];
    let first_blue= &color_1[4..];

    let second_red = &color_2[..2];
    let second_green= &color_2[2..4];
    let second_blue= &color_2[4..];

    let mean_red = mean_hex(first_red, second_red);
    let mean_green = mean_hex(first_green, second_green);
    let mean_blue = mean_hex(first_blue, second_blue);
    format!("{}{}{}", mean_red, mean_green, mean_blue)
}

fn main() {
    let args = Args::parse();
    let mean_color = find_mean_color(args.color_1.as_str(), args.color_2.as_str());
    println!("Mean Color => {}", mean_color);
}
