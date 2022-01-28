mod algorithm;
mod file;

fn main() {
    let arr = [42, 28];
    println!(
        "the value of GCD {}, and {} is {}.",
        arr[0],
        arr[1],
        algorithm::gcd::gcd(arr[0], arr[1])
    );

    let poem = file::text::read("./README.md");
    println!("{}", poem);

    let _bmp = file::bmp::read("./assets/sample_640×426.bmp");
}
