fn even_odd(n : i32) {
    for i in 0..n {
        if i % 2 == 0 {
            println!("even {}", i);
        } else {
            println!("odd {}", i);
        }
    }
}

fn main() {
    even_odd(20);
}
