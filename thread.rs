use std::thread;

fn main() {
    let mut v = vec![];

    let handle = thread::spawn(move || {
        println!("vector {:?}", v);
    });

    println!("{:?}", v);
    handle.join();
}