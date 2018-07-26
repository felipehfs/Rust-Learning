// The program return the number module 
fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

fn main() {
   println!("{}", abs(-1.0));
}
