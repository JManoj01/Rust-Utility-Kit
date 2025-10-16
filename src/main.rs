mod commands;

use clap::{Parser, Subcommand};
use commands::*;

#[derive(Parser)]
#[command(name = "toolbox")]
#[command(about = "Rust Utility Toolbox", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 🔗 URL Shortener - Create and manage short URLs locally
    Url {
        #[command(subcommand)]
        action: url_shortener::UrlAction,
    },
    
    /// 🔐 Password Generator - Generate secure random passwords
    Password {
        #[arg(short, long, default_value = "16")]
        length: usize,
        
        #[arg(short, long)]
        symbols: bool,
        
        #[arg(short, long)]
        qr: bool,
    },
    
    /// 🧾 QR Code Generator - Generate QR codes from text or URLs
    Qr {
        /// Text or URL to encode
        text: String,
        
        /// Output file path (optional, defaults to qrcode.png)
        #[arg(short, long)]
        output: Option<String>,
        
        /// Display as ASCII in terminal
        #[arg(short, long)]
        ascii: bool,
    },
    
    /// 🗂️ File Organizer - Organize files by type
    Organize {
        path: String,
        
        #[arg(short, long)]
        dry_run: bool,
    },
    
    /// 🧮 Base Converter - Convert between number bases
    Base {
        value: String,
        
        #[arg(short, long)]
        from: String,
        
        #[arg(short, long)]
        to: String,
    },
    
    /// 🕵️ Text Hasher - Hash strings with various algorithms
    Hash {
        text: String,
        
        #[arg(short, long, default_value = "sha256")]
        algorithm: String,
    },
}

fn main() {
    let cli = Cli::parse();
    
    let result = match cli.command {
        Commands::Url { action } => url_shortener::handle(action),
        Commands::Password { length, symbols, qr } => password_gen::handle(length, symbols, qr),
        Commands::Qr { text, output, ascii } => qr_generator::handle(&text, output, ascii),
        Commands::Organize { path, dry_run } => file_organizer::handle(&path, dry_run),
        Commands::Base { value, from, to } => base_converter::handle(&value, &from, &to),
        Commands::Hash { text, algorithm } => text_hasher::handle(&text, &algorithm),
    };
    
    if let Err(e) = result {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }
}
