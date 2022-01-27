use std::fs::File;
use std::io::prelude::*;

pub fn read(path: &str) -> String {
    println!("Start road file: {}", path);

    let mut f = File::open(path).expect("file not found!");

    let mut texts = String::new();

    f.read_to_string(&mut texts)
        .expect("something went wrong reading the file 😇");

    println!("finished read text.");
    texts
}
