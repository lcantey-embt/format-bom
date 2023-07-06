#![allow(unused)]

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

/// remove utf-8 BOM mark of given file
fn remove_bom<P: AsRef<Path>>(path: P) -> Result<()> {
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

/// add utf-8 BOM mark to given file if the file is utf-8 encoded
fn add_bom<P: AsRef<Path>>(path: P) -> Result<()> {
    let mut reader = BufReader::new(File::open(&path)?);

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    // Check if the file is UTF-8 encoded
    let utf8_check = String::from_utf8(buf.clone());
    if utf8_check.is_err() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "File is not UTF-8 encoded",
        ));
    }

    let mut bom_bytes = [0; 3];
    bom_bytes.copy_from_slice(&buf[0..3]);

    let mut contents = Vec::new();
    if bom_bytes != [0xEF, 0xBB, 0xBF] {
        contents.extend_from_slice(&[0xEF, 0xBB, 0xBF]); // add BOM
        contents.extend_from_slice(&buf); // add the original contents
    } else {
        contents.extend_from_slice(&buf); // if BOM was already there, just add original contents
    }

    let mut writer = BufWriter::new(File::create(path)?);
    writer.write_all(&contents)?;

    Ok(())
}

fn is_buf_utf8(buf: &Vec<u8>) -> Result<bool> {
    let utf8_check = String::from_utf8(buf.clone());
    if utf8_check.is_err() {
        return Ok(false);
    }

    Ok(true)
}

fn is_utf8<P: AsRef<Path>>(path: P) -> Result<bool> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
