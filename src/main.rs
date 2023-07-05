#![allow(unused)]
use clap::Parser;

fn main() {
    let args = Args::parse();
}

/// Simple program to greet a person
/// ```rust
/// #[derive(Parser, Debug)]
/// #[command(author, version, about, long_about = None)]
/// struct Args {
///     /// Name of the person to greet
///     #[arg(short, long)]
///     name: String,
///
///     /// Number of times to greet
///     #[arg(short, long, default_value_t = 1)]
///     count: u8,
/// }
/// ```
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}
