use std::collections::HashMap;

fn main() {
    let mut fruit_map = HashMap::new();
    fruit_map.insert("apple", 5);
    fruit_map.insert("banana", 3);
    fruit_map.insert("orange", 4);

    println!("{:?}", fruit_map);
    if !fruit_map.contains_key("kiwi") {
        for (k, v) in &fruit_map {
            println!("fruit={}, rating={}", k, v);
        }
        println!("We got {} fruits but not kiwi", fruit_map.len());
    }

    let fruits = vec!["banana", "kiwi", "apple", "orange"];

    for &fruit in fruits.iter() {
        match fruit_map.get(fruit) {
            Some(f) => println!("{}: {}", fruit, f),
            None => println!("{} is not ranked", fruit),
        }
    }
}
