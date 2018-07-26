fn main() {
    let f = |x| x + 1;
    let x = 10;
    println!("{}", f(x));
    let p = || println!("This is a closure");
    p();

}

