// There are 2 type of string the static and 
// the type string. So, 
// The type String under the hood,
// represents vec<u8>

fn dump(s: &str) {
    println!("str {}", s);
}

fn main() {
    let text = "hello dolly";
    let s = text.to_string();

    dump(text);
    dump(&s);
}
