#![allow(unused)]

mod checker;
mod fixer;

use clap::Parser;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Result, Write};
use std::path::Path;

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
struct Args {
    path: Option<String>,
    #[arg(short, long, required = false)]
    recursive: bool,
    #[arg(long, required = false)]
    option: Option<String>,
}
