use std::fs::File;
use std::io::{self, prelude::*};

fn main() {
    println!("Write file name: ");
    let mut input_value = String::new();

    io::stdin()
        .read_line(&mut input_value)
        .expect("Error reading input value");

    let input_value = input_value.trim();

    let mut filepath = String::from("./txts/");

    let filename = format!("{}.txt", input_value);

    filepath.push_str(&filename);

    let mut file = File::create(filepath).expect("Error creating file in fs");

    println!("Write file content: ");
    let mut text_value = String::new();

    io::stdin()
        .read_line(&mut text_value)
        .expect("Error reading input value");

    file.write_all(text_value.as_bytes())
        .expect("Error writing data to file");
}
