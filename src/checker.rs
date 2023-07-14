#![allow(unused)]

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn is_buf_utf8(buf: &Vec<u8>) -> Result<bool, Box<dyn Error>> {
    let utf8_check = String::from_utf8(buf.clone());
    if utf8_check.is_err() {
        return Ok(false);
    }

    Ok(true)
}

pub fn is_utf8<P: AsRef<Path>>(path: P) -> Result<bool, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
