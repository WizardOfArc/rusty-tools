use std::env;
use std::process;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <color-code>", args[0]);
        eprintln!("Examples: #efc, #564da9, rgb(123, 67, 254)");
        process::exit(1);
    }

    let color_code = &args[1];
    
    if let Some(result) = convert_color(color_code) {
        println!("{}", result);
    } else {
        eprintln!("Invalid color format. Use #efc, #564da9, or rgb(123, 67, 254)");
        process::exit(1);
    }
}

fn convert_color(color_code: &str) -> Option<String> {
    // Regex for hex codes (both 3 and 6 digits)
    let hex_regex = Regex::new(r"^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6})$").unwrap();
    
    // Regex for RGB format
    let rgb_regex = Regex::new(r"^rgb\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\)$").unwrap();
    
    if hex_regex.is_match(color_code) {
        // Convert hex to RGB
        hex_to_rgb(color_code)
    } else if rgb_regex.is_match(color_code) {
        // Convert RGB to hex
        rgb_to_hex(color_code)
    } else {
        None
    }
}

fn hex_to_rgb(hex: &str) -> Option<String> {
    // Remove the # prefix
    let hex = &hex[1..];
    
    let (r, g, b) = if hex.len() == 3 {
        // Convert 3-digit hex to 6-digit equivalent
        let r = u8::from_str_radix(&format!("{}{}", &hex[0..1], &hex[0..1]), 16).ok()?;
        let g = u8::from_str_radix(&format!("{}{}", &hex[1..2], &hex[1..2]), 16).ok()?;
        let b = u8::from_str_radix(&format!("{}{}", &hex[2..3], &hex[2..3]), 16).ok()?;
        (r, g, b)
    } else {
        // Parse 6-digit hex
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        (r, g, b)
    };
    
    Some(format!("rgb({}, {}, {})", r, g, b))
}

fn rgb_to_hex(rgb: &str) -> Option<String> {
    let rgb_regex = Regex::new(r"^rgb\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\)$").unwrap();
    
    if let Some(caps) = rgb_regex.captures(rgb) {
        let r: u8 = caps[1].parse().ok()?;
        let g: u8 = caps[2].parse().ok()?;
        let b: u8 = caps[3].parse().ok()?;
        
        Some(format!("#{:02x}{:02x}{:02x}", r, g, b))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb_3digit() {
        assert_eq!(hex_to_rgb("#efc"), Some("rgb(238, 255, 204)".to_string()));
    }

    #[test]
    fn test_hex_to_rgb_6digit() {
        assert_eq!(hex_to_rgb("#564da9"), Some("rgb(86, 77, 169)".to_string()));
    }

    #[test]
    fn test_rgb_to_hex() {
        assert_eq!(rgb_to_hex("rgb(123, 67, 254)"), Some("#7b43fe".to_string()));
    }

    #[test]
    fn test_rgb_to_hex_spaces() {
        assert_eq!(rgb_to_hex("rgb(34, 231, 0)"), Some("#22e700".to_string()));
    }
}
