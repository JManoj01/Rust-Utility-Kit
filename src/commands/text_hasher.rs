use crate::commands::Result;
use blake3;
use colored::Colorize;
use md5;
use sha2::{Digest, Sha256, Sha512};

pub fn handle(text: &str, algorithm: &str) -> Result<()> {
    let algo_lower = algorithm.to_lowercase();
    
    let hash = match algo_lower.as_str() {
        "md5" => {
            let digest = md5::compute(text.as_bytes());
            format!("{:x}", digest)
        }
        
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(text.as_bytes());
            format!("{:x}", hasher.finalize())
        }
        
        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.update(text.as_bytes());
            format!("{:x}", hasher.finalize())
        }
        
        "blake3" => {
            blake3::hash(text.as_bytes()).to_string()
        }
        
        _ => {
            return Err(format!(
                "Unsupported algorithm: {}. Use md5, sha256, sha512, or blake3",
                algorithm
            ).into());
        }
    };
    
    println!("{}", "🕵️  Hash Result:".green().bold());
    println!("   Algorithm: {}", algo_lower.bright_cyan());
    println!("   Input:     {}", text);
    println!("   Hash:      {}", hash.bright_green().bold());
    
    Ok(())
}
