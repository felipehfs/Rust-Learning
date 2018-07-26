// modifies the value 
// using the mutable borrowing
fn modifies(x: &mut f64) {
    *x = 1.0;
}

fn main() {
    let mut res = 0.0;
    modifies(&mut res);
    println!("Res is {}", res);
}
