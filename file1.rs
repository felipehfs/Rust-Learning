use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let first = env::args().nth(1).expect("please supply a filename");
    let mut file = File::open(&first).expect("can't open the file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("Can't read the file");

    println!("File had {} bytes", text.len());
}
