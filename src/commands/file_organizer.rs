use crate::commands::Result;
use colored::Colorize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn handle(path: &str, dry_run: bool) -> Result<()> {
    let source_path = Path::new(path);
    
    if !source_path.exists() {
        return Err(format!("Path does not exist: {}", path).into());
    }
    
    if !source_path.is_dir() {
        return Err("Path must be a directory".into());
    }
    
    let categories = get_file_categories();
    let mut moves: Vec<(PathBuf, PathBuf)> = Vec::new();
    
    for entry in WalkDir::new(source_path)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry_path = entry.path();
        
        if entry_path.is_file() {
            if let Some(ext) = entry_path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                
                for (category, extensions) in &categories {
                    if extensions.contains(&ext_str.as_str()) {
                        let mut target_dir = source_path.to_path_buf();
                        target_dir.push(category);
                        
                        let file_name = entry_path.file_name().unwrap();
                        let target_path = target_dir.join(file_name);
                        
                        moves.push((entry_path.to_path_buf(), target_path));
                        break;
                    }
                }
            }
        }
    }
    
    if moves.is_empty() {
        println!("{}", "📭 No files to organize".yellow());
        return Ok(());
    }
    
    println!("{}", "🗂️  File Organization Plan:".green().bold());
    
    let mut category_counts: HashMap<String, usize> = HashMap::new();
    
    for (source, target) in &moves {
        let category = target.parent().unwrap().file_name().unwrap().to_string_lossy();
        *category_counts.entry(category.to_string()).or_insert(0) += 1;
        
        if dry_run {
            println!(
                "   {} → {}",
                source.file_name().unwrap().to_string_lossy(),
                category.bright_cyan()
            );
        }
    }
    
    println!("\n📊 Summary:");
    for (category, count) in category_counts {
        println!("   {}: {} files", category.bright_cyan(), count);
    }
    
    if dry_run {
        println!("\n{}", "🔍 DRY RUN - No changes made".yellow().bold());
        println!("   Remove --dry-run flag to apply changes");
    } else {
        for (source, target) in moves {
            let target_dir = target.parent().unwrap();
            fs::create_dir_all(target_dir)?;
            fs::rename(source, target)?;
        }
        println!("\n{}", "✅ Files organized successfully!".green().bold());
    }
    
    Ok(())
}

fn get_file_categories() -> HashMap<&'static str, Vec<&'static str>> {
    let mut categories = HashMap::new();
    
    categories.insert(
        "Documents",
        vec!["pdf", "doc", "docx", "txt", "odt", "rtf", "tex", "md"],
    );
    
    categories.insert(
        "Images",
        vec!["jpg", "jpeg", "png", "gif", "bmp", "svg", "webp", "ico"],
    );
    
    categories.insert(
        "Videos",
        vec!["mp4", "avi", "mkv", "mov", "wmv", "flv", "webm"],
    );
    
    categories.insert(
        "Audio",
        vec!["mp3", "wav", "flac", "aac", "ogg", "wma", "m4a"],
    );
    
    categories.insert(
        "Archives",
        vec!["zip", "rar", "7z", "tar", "gz", "bz2", "xz"],
    );
    
    categories.insert(
        "Code",
        vec!["rs", "py", "js", "ts", "java", "c", "cpp", "h", "go", "rb"],
    );
    
    categories.insert(
        "Spreadsheets",
        vec!["xls", "xlsx", "csv", "ods"],
    );
    
    categories
}
