use crate::commands::Result;
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum UrlAction {
    /// Add a new URL
    Add {
        /// URL to shorten
        url: String,
    },
    
    /// Get original URL from short code
    Get {
        /// Short code
        code: String,
    },
    
    /// List all URLs
    List,
    
    /// Open URL in browser
    Open {
        /// Short code
        code: String,
    },
}

#[derive(Serialize, Deserialize)]
struct UrlMapping {
    urls: HashMap<String, String>,
}

fn get_storage_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(".rust-toolbox");
    fs::create_dir_all(&path).ok();
    path.push("urls.json");
    path
}

fn load_urls() -> Result<UrlMapping> {
    let path = get_storage_path();
    
    if path.exists() {
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    } else {
        Ok(UrlMapping {
            urls: HashMap::new(),
        })
    }
}

fn save_urls(mapping: &UrlMapping) -> Result<()> {
    let path = get_storage_path();
    let content = serde_json::to_string_pretty(mapping)?;
    fs::write(path, content)?;
    Ok(())
}

fn generate_short_code(url: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    let result = hasher.finalize();
    hex::encode(&result[..4]) // Use first 4 bytes for short code
}

pub fn handle(action: UrlAction) -> Result<()> {
    match action {
        UrlAction::Add { url } => {
            let mut mapping = load_urls()?;
            let code = generate_short_code(&url);
            mapping.urls.insert(code.clone(), url.clone());
            save_urls(&mapping)?;
            println!("✅ URL shortened!");
            println!("   Original: {}", url);
            println!("   Code: {}", code);
        }
        
        UrlAction::Get { code } => {
            let mapping = load_urls()?;
            match mapping.urls.get(&code) {
                Some(url) => println!("🔗 {}", url),
                None => println!("❌ No URL found for code: {}", code),
            }
        }
        
        UrlAction::List => {
            let mapping = load_urls()?;
            if mapping.urls.is_empty() {
                println!("📭 No URLs stored yet");
            } else {
                println!("📋 Stored URLs:");
                for (code, url) in mapping.urls.iter() {
                    println!("   {} → {}", code, url);
                }
            }
        }
        
        UrlAction::Open { code } => {
            let mapping = load_urls()?;
            match mapping.urls.get(&code) {
                Some(url) => {
                    println!("🌐 Opening: {}", url);
                    webbrowser::open(url)?;
                }
                None => println!("❌ No URL found for code: {}", code),
            }
        }
    }
    
    Ok(())
}
