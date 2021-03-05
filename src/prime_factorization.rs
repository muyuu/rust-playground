
#[test]
fn test_prime_factorization() {
    let one = prime_factorization(1);
    assert_eq!(one, Vec::new());

    let zero = prime_factorization(0);
    assert_eq!(zero, Vec::new());

    let r = prime_factorization(360);
    let mut expected = Vec::new();
    expected.push(2);
    expected.push(2);
    expected.push(2);
    expected.push(3);
    expected.push(3);
    expected.push(5);
    assert_eq!(r, expected);

    let prime_number = prime_factorization(343433);
    let mut expected = Vec::new();
    expected.push(343433);
    assert_eq!(prime_number, expected);
}

pub fn prime_factorization(a: i32) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut divide_number = 2;
    let mut base_number = a;

    if base_number <= 1 {
        return vec;
    }

    loop {
        if base_number / divide_number < 1 {
            break;
        }

        let result = base_number % divide_number;

        if result == 0 {
            vec.push(divide_number);
            base_number = base_number / divide_number;
        } else {
            divide_number += 1;
        }
    }

    vec
}
