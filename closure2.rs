fn main() {
    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("The count incremented by 1: {}", count);
    };

    inc();
    inc();
    inc();

}
