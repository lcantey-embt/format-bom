#![allow(unused)]

use clap::Parser;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
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
struct Args {}

/// remove utf-8 BOM mark of given file
fn remove_bom<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    let mut reader = BufReader::new(File::open(&path)?);

    let mut buf = [0; 3];
    reader.read_exact(&mut buf)?;

    let mut contents = Vec::new();
    if buf != [0xEF, 0xBB, 0xBF] {
        contents.extend_from_slice(&buf);
    }
    reader.read_to_end(&mut contents)?;

    let mut writer = BufWriter::new(File::create(path)?);
    writer.write_all(&contents)?;

    Ok(())
}
