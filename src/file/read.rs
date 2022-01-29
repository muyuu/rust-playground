use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;

pub fn read_txt(path: &str) -> Result<String> {
    let content = std::fs::read_to_string(path).with_context(|| format!("File Not Found"))?;
    Ok(content)
}

pub fn read_bin(path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(path).with_context(|| format!("File Not Found"))?;
    let mut buf = Vec::new();

    file.read_to_end(&mut buf)
        .with_context(|| format!("something went wrong reading the file"))?;
    Ok(buf)
}
