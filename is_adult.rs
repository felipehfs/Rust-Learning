fn is_adult(age: u32)  {
    match age {
        0...12 => println!("Criança"),
        13...21 => println!("Adolescente"),
        22...30 => println!("Adulto"),
        _ => println!("Idoso")
    }
}


fn main() {
    is_adult(23);
}

