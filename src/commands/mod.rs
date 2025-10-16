pub mod url_shortener;
pub mod password_gen;
pub mod qr_generator;
pub mod file_organizer;
pub mod base_converter;
pub mod text_hasher;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
