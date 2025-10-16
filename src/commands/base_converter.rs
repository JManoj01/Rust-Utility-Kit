use crate::commands::Result;
use colored::Colorize;
use base64::prelude::*;

pub fn handle(value: &str, from: &str, to: &str) -> Result<()> {
    let from_lower = from.to_lowercase();
    let to_lower = to.to_lowercase();
    
    let result = match (from_lower.as_str(), to_lower.as_str()) {
        ("text", "base64") => BASE64_STANDARD.encode(value),
        
        ("base64", "text") => {
            let decoded = BASE64_STANDARD.decode(value)?;
            String::from_utf8(decoded)?
        }
        
        _ => {
            let decimal = parse_to_decimal(value, &from_lower)?;
            format_from_decimal(decimal, &to_lower)?
        }
    };
    
    println!("{}", "🧮 Conversion Result:".green().bold());
    println!("   Input:  {} ({})", value.bright_cyan(), from);
    println!("   Output: {} ({})", result.bright_green().bold(), to);
    
    Ok(())
}

fn parse_to_decimal(value: &str, base: &str) -> Result<i64> {
    match base {
        "binary" | "bin" => {
            i64::from_str_radix(value.trim_start_matches("0b"), 2)
                .map_err(|e| format!("Invalid binary: {}", e).into())
        }
        "decimal" | "dec" => {
            value.parse::<i64>()
                .map_err(|e| format!("Invalid decimal: {}", e).into())
        }
        "hex" | "hexadecimal" => {
            i64::from_str_radix(value.trim_start_matches("0x"), 16)
                .map_err(|e| format!("Invalid hex: {}", e).into())
        }
        "octal" | "oct" => {
            i64::from_str_radix(value.trim_start_matches("0o"), 8)
                .map_err(|e| format!("Invalid octal: {}", e).into())
        }
        _ => Err(format!("Unsupported source base: {}. Use binary, decimal, hex, octal, base64, or text", base).into()),
    }
}

fn format_from_decimal(value: i64, base: &str) -> Result<String> {
    match base {
        "binary" | "bin" => Ok(format!("0b{:b}", value)),
        "decimal" | "dec" => Ok(value.to_string()),
        "hex" | "hexadecimal" => Ok(format!("0x{:x}", value)),
        "octal" | "oct" => Ok(format!("0o{:o}", value)),
        _ => Err(format!("Unsupported target base: {}. Use binary, decimal, hex, octal, base64, or text", base).into()),
    }
}
