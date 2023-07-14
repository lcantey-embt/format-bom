use crate::checker;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

/// remove utf-8 BOM mark of given file
pub fn remove_bom(path: &PathBuf) -> Result<bool, Box<dyn Error>> {
    let mut reader = get_file_reader(path)?;

    let mut buf = vec![0; BOM.len()];
    reader.read_exact(&mut buf)?;

    if &buf != BOM {
        return Ok(false);
    }

    let mut temp_file = NamedTempFile::new()?;
    {
        let mut writer = BufWriter::new(&mut temp_file);
        io::copy(&mut reader, &mut writer)?;
    }
    temp_file.persist(path)?;
    println!("Removed BOM from {}", path.display());
    Ok(true)
}

/// add utf-8 BOM mark to given file if the file is utf-8 encoded
pub fn add_bom(path: &PathBuf) -> Result<bool, Box<dyn Error>> {
    let mut reader = get_file_reader(path)?;

    let mut buf = vec![0; BOM.len()];

    reader.read_exact(&mut buf)?;

    if &buf == BOM {
        return Ok(false);
    }

    reader.read_to_end(&mut buf)?;
    if checker::is_buf_utf8(&buf)? == false {
        return Ok(false);
    }

    let mut temp_file = NamedTempFile::new()?;
    {
        let mut writer = BufWriter::new(&mut temp_file);
        writer.write_all(BOM)?;
        writer.write_all(&buf)?;
    }
    temp_file.persist(&path)?;
    println!("Added BOM to {}", path.display());
    Ok(true)
}

const BOM: &[u8] = b"\xEF\xBB\xBF";

fn get_file_reader(path: &Path) -> Result<BufReader<File>, Box<dyn Error>> {
    File::open(path)
        .and_then(|file| Ok(BufReader::new(file)))
        .map_err(|e| e.into())
}
