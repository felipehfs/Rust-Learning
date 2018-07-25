#[derive(Debug)]
struct A {
    s: &'static str
}

fn main() {
    let a = A{s: "Hello-dammit"};

    println!("{:?}", a);
}
