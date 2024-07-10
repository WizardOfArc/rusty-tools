use std::num::ParseIntError;

use clap::Parser;

#[derive(Debug)]
struct HexError<'a>
{
    hex: &'a str,
    message: String
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  color_1: String,
  color_2: String,
}

fn mean_hex(hex_1: &str, hex_2: &str) -> Result<String, ParseIntError>{
    let num1 = u16::from_str_radix(hex_1, 16)?;
    let num2 = u16::from_str_radix(hex_2, 16)?;
    let mean = (num1 + num2) / 2;
    Ok(format!("{:x}", mean))
}

fn find_mean_color(color_1: &str, color_2: &str) ->  Result<String, ParseIntError> {
    let first_red = &color_1[..2];
    let first_green= &color_1[2..4];
    let first_blue= &color_1[4..];

    let second_red = &color_2[..2];
    let second_green= &color_2[2..4];
    let second_blue= &color_2[4..];

    let mean_red = mean_hex(first_red, second_red)?;
    let mean_green = mean_hex(first_green, second_green)?;
    let mean_blue = mean_hex(first_blue, second_blue)?;
    Ok(format!("{}{}{}", mean_red, mean_green, mean_blue))
}

fn hex_valid(hex: &str) -> Result<&str, HexError> {
    if hex.len() != 6 {
        return Err(HexError{hex, message: "hex length needs to be 6".to_string()});
    }
    for c in hex.chars() {
        if !c.is_ascii_hexdigit() {
            return Err(
                HexError{hex, message: format!("Invalid hex digit '{}'", c)}
            );
        }
    };
    Ok(hex)
}

fn validate_hex_pair<'a>(hex_1: &'a str, hex_2: &'a str) -> Result<(&'a str, &'a str), HexError<'a>> {
    hex_valid(hex_1)?;
    hex_valid(hex_2)?;
    Ok((hex_1, hex_2))
}

fn main() {
    let args = Args::parse();

    match validate_hex_pair(&args.color_1, &args.color_2) {
        Ok(color_pair) => {
        find_mean_color(color_pair.0, color_pair.1)
            .and_then(|color| {
                println!("Mean Color => {}",color);
                Ok(())
            }
            ).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
            });

        },
        Err(e) => {
            eprintln!("Error: {}  [{}]", e.message, e.hex);
            return;
        }
    }
}
