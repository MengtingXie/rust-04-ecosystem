use anyhow::Context;
use ecosystem::MyError;
use std::{fs, mem::size_of};
fn main() -> Result<(), anyhow::Error> {
    println!("size of MyError is {}", std::mem::size_of::<MyError>());
    println!("size of std::io::Error is {}", size_of::<std::io::Error>());
    println!(
        "size of std::num::ParseIntError is {}",
        size_of::<std::num::ParseIntError>()
    );
    println!(
        "size of serde_json::Error is {}",
        size_of::<serde_json::Error>()
    );

    println!("size of std::io::Error is {}", size_of::<std::io::Error>());
    println!("size of MyError is {}", size_of::<MyError>());

    let filename: &str = "non-existent-file.txt";
    let _fd =
        fs::File::open(filename).with_context(|| format!("Can't not find file: {}", filename))?;

    fail_with_error()?;

    Ok(())
}

fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::Custom("This is a custom error".to_string()))
}
