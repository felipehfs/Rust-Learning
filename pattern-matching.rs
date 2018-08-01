fn fizz_buzz(n:i32) -> Option<String> {
    match(n % 3, n % 5 ) {
        (0, 0) => Some("Fizzbuzz".to_owned()),
        (0, _ ) => Some("Fizz".to_owned()),
        (_, 0) =>  Some("Buzz".to_owned()),
        _ => None
    }
}

fn main() {
    for i in 0..100 {
        let f = fizz_buzz(i);
        if f.is_some() {
            println!("{}", f.unwrap());
        }
    }
}
