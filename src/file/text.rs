use crate::file::read::read_txt;
use anyhow::Result;

pub fn read(path: &str) -> Result<String> {
    let t = read_txt(path);
    t
}
