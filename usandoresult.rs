fn is_even(n: i32) -> Result<bool, bool> {
    if n % 2 == 0 {
        Ok(true)
    } else {
        Err(false)
    }
}
fn main() {
    let p = is_even(30);
    println!("{:?}", p);
}