// A function which takes a closure as an argument and calls it
fn apply<F>(f: F) where 
    // The closure takes no input and returns nothings
    F:FnOnce() {
        f();
}

fn apply_to_28<F>(f: F) -> i32 where 
    F: Fn(i32) -> i32 {
    f(3)    
}

fn main() {
    
    let greeting = "hello";
    let say_hello = || {
        println!("Hello world");
    };
    
    let double = |x| 2 * x;
    apply(say_hello);
    println!("3 doubled: {}", apply_to_28(double));
}
