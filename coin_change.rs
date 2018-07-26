// The greedy algorihtm - The coin change problem
use std::collections::HashMap;

/// coin_calculate return the quantity of change of each coin
fn coin_calculate(value: u32) {
    
    let coins:[u32; 5] = [50, 25, 10, 5, 1];
    let mut results = HashMap::new();
    let mut total = value;
    
    for coin in coins.iter() {
        let qtd = total / coin;
        results.insert(coin, qtd);
        total %= coin;
    }
    
    println!("{:?}", results);
}

fn main() {
    coin_calculate(60);
}

