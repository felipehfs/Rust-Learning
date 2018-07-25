fn sum(values: &[i32]) -> i32 {
    let mut result = 0;
    for value in 0..values.len() {
        result += values[value]
    }
    result 
}

fn main() {
    let arr = [10, 20, 30, 40, 50, 60];
    let res = sum(&arr);
    println!("{:?}", res);
}
