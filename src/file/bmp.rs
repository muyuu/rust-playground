use std::fs::File;
use std::io::Read;

pub fn read(path: &str) -> Vec<u8> {
    let mut file = File::open(path).expect("file not found!");

    let mut buf = Vec::new();
    file.read_to_end(&mut buf)
        .expect("something went wrong reading the file");

    println!("finished read binary");

    if is_bmp(&buf) == false {
        panic!("It's not BMP file 😇");
    }

    buf
}

pub fn is_bmp(buf: &Vec<u8>) -> bool {
    let first_byte = buf[0];
    let second_byte = buf[1];

    if first_byte == 0x42 && second_byte == 0x4d {
        return true;
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gcd() {
        let buf1 = vec![0x42, 0x4d, 0x11];
        let buf2 = vec![0x42, 0x43, 0x11];
        assert_eq!(is_bmp(&buf1), true);
        assert_eq!(is_bmp(&buf2), false);
    }
}
