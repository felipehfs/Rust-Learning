use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("exercicio55.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    println!("{}", contents);
}
