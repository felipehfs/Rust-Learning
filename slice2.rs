fn main() {
    let ints = [1, 2, 3, 4, 5, 6];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(6);

    println!("first {:?}", first);
    println!("last {:?}", last);
}
