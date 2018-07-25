fn main() {
    let multilingual = "Hi! Hola!";
    for ch in multilingual.chars() {
        print!("{}", ch);
    }
    println!("");
    println!("len {}", multilingual.len());

    let maybe = multilingual.find('n');
    if maybe.is_some() {
        let hi = &multilingual[maybe.unwrap()..];
        println!("Russian h1 {}", hi);
    }
}
