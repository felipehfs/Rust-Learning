fn is_even(n: i32) -> Option<i32> {
    if n % 2 == 0 {
        Some(n)
    } else {
        None
    }
}

fn main() {
    for i in 1..30 {
        if is_even(i).is_some() {
            println!("{}", i);
        }
    }
}
