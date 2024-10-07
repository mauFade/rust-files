use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("file.txt").expect("Error reading file from fs");

    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("Error when serialazing file to string");

    println!("File content:\n\n{}", content);
}
