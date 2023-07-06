use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Result, Write};
use std::path::Path;

/// remove utf-8 BOM mark of given file
pub fn remove_bom<P: AsRef<Path>>(path: P) -> Result<()> {
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
pub fn add_bom<P: AsRef<Path>>(path: P) -> Result<()> {
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
