// The high order function the function as parameter
fn run<F>(f: F) where F: Fn() {
    f()
}

// The closure can be a struct attribute
#[derive(Debug)]
struct A<F: Fn(i32) -> i32> {
    f:F
}

fn main() {
    let p = || println!("Hello from run function!");
    run(p);

    let x = |i| i + 10;
    let a = A{
        f: x,
    };

}
