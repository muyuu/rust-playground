use std::fs::File;
use std::io::Read;

pub fn read_txt(path: &str) -> String {
    println!("Start rad file: {}", path);

    let mut f = File::open(path).expect("file not found");
    let mut t = String::new();
    f.read_to_string(&mut t)
        .expect("something went wrong reading the file");

    println!("finished read text");
    t
}

pub fn read_bin(path: &str) -> Vec<u8> {
    let mut file = File::open(path).expect("file not found!");
    let mut buf = Vec::new();

    file.read_to_end(&mut buf)
        .expect("something went wrong reading the file");

    println!("finished read binary");

    buf
}
