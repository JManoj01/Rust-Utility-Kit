# Rust-Utility-Kit

An all-in-one CLI toolkit for developers and students.

## Features

**🔗 URL Shortener** - Create and manage shortened URLs locally with hash-based codes

**🔐 Password Generator** - Generate secure random passwords with optional QR code output

**🧾 QR Code Generator** - Convert text or URLs into QR codes (PNG or ASCII)

**🗂️ File Organizer** - Automatically organize messy directories by file type

**🧮 Base Converter** - Convert between binary, decimal, hex, and base64

**🕵️ Text Hasher** - Hash strings using MD5, SHA256, SHA512, or BLAKE3

## 🚀 Installation

Clone the repository
git clone https://github.com/YOUR_USERNAME/rust-toolbox.git
cd rust-toolbox

Build and install
cargo install --path .

text

## 📖 Usage

Generate a password
rust-toolbox password --length 20 --symbols

Create a QR code
rust-toolbox qr "Hello World!" --ascii

Organize files
rust-toolbox organize ~/Downloads --dry-run

Convert bases
rust-toolbox base 255 --from decimal --to hex

Hash text
rust-toolbox hash "mypassword" --algorithm sha256

Shorten URLs
rust-toolbox url add "https://example.com"

text

Run `rust-toolbox --help` for more commands and options.

## 🛠️ Built With

- [Rust](https://www.rust-lang.org/) - Programming language
- [Clap](https://github.com/clap-rs/clap) - CLI parsing
- [QRCode](https://crates.io/crates/qrcode) - QR code generation
- [BLAKE3](https://github.com/BLAKE3-team/BLAKE3) - Fast cryptographic hashing

All Rights Reserved 
Justin Manoj
