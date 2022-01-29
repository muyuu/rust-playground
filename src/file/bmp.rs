use crate::file::read::read_bin;

pub fn read(path: &str) -> Vec<u8> {
    let bmp = read_bin(path);

    if is_bmp(&bmp) == false {
        panic!("It's not BMP file 😇");
    }

    bmp
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
