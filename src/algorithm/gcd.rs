pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }

    let c = gcd(b, a % b);
    c
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gcd() {
        let arr = [
            [200, 95, 5],
            [100, 10, 10],
            [12, 18, 6],
            [42, 28, 14],
            [27, 36, 9],
            [296, 185, 37],
            [265, 371, 53],
            [553, 237, 79],
        ];

        for i in arr {
            assert_eq!(
                gcd(i[0], i[1]),
                i[2],
                "{} と {} の最大公約数は {}",
                i[0],
                i[1],
                i[2]
            );
        }
    }
}
