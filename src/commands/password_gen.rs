use crate::commands::Result;
use colored::Colorize;
use rand::Rng;

const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()_+-=[]{}|;:,.<>?";

pub fn handle(length: usize, use_symbols: bool, generate_qr: bool) -> Result<()> {
    if length < 4 {
        return Err("Password length must be at least 4 characters".into());
    }
    
    let mut charset = String::new();
    charset.push_str(LOWERCASE);
    charset.push_str(UPPERCASE);
    charset.push_str(NUMBERS);
    
    if use_symbols {
        charset.push_str(SYMBOLS);
    }
    
    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect();
    
    println!("{}", "🔐 Generated Password:".green().bold());
    println!("   {}", password.bright_cyan().bold());
    
    // Calculate strength
    let strength = calculate_strength(length, use_symbols);
    println!("   Strength: {}", strength);
    
    if generate_qr {
        println!("\n📱 Generating QR code...");
        crate::commands::qr_generator::handle(&password, Some("password_qr.png".to_string()), false)?;
    }
    
    Ok(())
}

fn calculate_strength(length: usize, has_symbols: bool) -> String {
    let score = length + if has_symbols { 10 } else { 0 };
    
    if score >= 20 {
        "💪 Very Strong".green().to_string()
    } else if score >= 15 {
        "👍 Strong".bright_green().to_string()
    } else if score >= 10 {
        "⚠️  Medium".yellow().to_string()
    } else {
        "❌ Weak".red().to_string()
    }
}
