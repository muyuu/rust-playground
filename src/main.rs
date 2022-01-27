mod algorithm;

fn main() {
    let arr = [42, 28];
    println!(
        "the value of GCD {}, and {} is {}.",
        arr[0],
        arr[1],
        algorithm::gcd::gcd(arr[0], arr[1])
    );
}
