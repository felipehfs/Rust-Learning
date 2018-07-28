fn main() {
    let numbers = [1, 2, 3, 4, 5];

    let result2 = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("result: {}", result2);
}
