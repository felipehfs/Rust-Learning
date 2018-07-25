use std::collections::HashMap;

fn main() {
    let mut contacts = HashMap::new();
    
    contacts.insert("daniel", "203-1244");
    contacts.insert("marcos", "221-4023");
    contacts.insert("katie", "093-2311");
    
    match contacts.get(&"daniel") {
        Some(&number) => println!("Calling Ashley: {}", number),
        _ => println!("Don't have Ashley's number."),
    }

    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, number);
    }
}
