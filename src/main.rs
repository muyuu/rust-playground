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

    let readme = file::text::read("./README.md");
    match readme {
        Ok(x) => println!("Read Success!!\n{}", x),
        Err(e) => println!("Error.\n{}", e),
    }

    let bmp = file::bmp::read("./assets/muyuu.png");
    match bmp {
        Ok(x) => println!("1st byte: {}, 2nd byte: {}.", x[0], x[1]),
        Err(x) => println!("T_T why?\n {:?}", x),
    }
}
