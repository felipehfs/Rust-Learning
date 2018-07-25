fn main() {
    let elem = 5u8;

    let mut vec = Vec::new();

    vec.push(elem);
    vec.push(2);
    vec.push(12);
    vec.push(20);
    vec.push(10);
    vec.sort();

    println!("{:?}", vec);
}
