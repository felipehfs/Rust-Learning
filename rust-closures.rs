// Closure is the important feature of functional programming

fn main(){
    let haystack = vec![1, 2, 3];
    // forces the entry of ownership to the closure
    let contains = move |needle| haystack.contains(needle);
    
    println!("{}", contains(&1));
    println!("{}", contains(&4));
}
