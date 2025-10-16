use crate::commands::Result;
use colored::Colorize;
use qrcode::QrCode;
use image::Luma;

pub fn handle(text: &str, output: Option<String>, ascii: bool) -> Result<()> {
    let code = QrCode::new(text.as_bytes())?;
    
    if ascii {
        let string = code
            .render::<char>()
            .quiet_zone(false)
            .module_dimensions(2, 1)
            .light_color(' ')
            .dark_color('█')
            .build();
        
        println!("{}", "🧾 QR Code:".green().bold());
        println!("{}", string);
    } else {
        let output_path = output.unwrap_or_else(|| "qrcode.png".to_string());
        let image = code.render::<Luma<u8>>().build();
        image.save(&output_path)?;
        
        println!("{}", "✅ QR Code generated!".green().bold());
        println!("   Saved to: {}", output_path.bright_cyan());
    }
    
    Ok(())
}
