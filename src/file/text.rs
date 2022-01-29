use crate::file::read::read_txt;

pub fn read(path: &str) -> String {
    let t = read_txt(path);
    t
}
