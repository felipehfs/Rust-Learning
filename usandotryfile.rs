use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = try!(File::open(String::from("usando_rust.txt")));
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    
}
