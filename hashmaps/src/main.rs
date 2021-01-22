use std::collections::HashMap;

fn show_basket(fruit_basket: &mut HashMap<String, u32>) {
    for (key, value) in fruit_basket {
        println!("key = {}, value = {}", key, value);
    }
}

fn main() {
    let mut fruit_basket: HashMap<String, u32> = HashMap::new();

    fruit_basket.insert(String::from("apple"), 10);
    fruit_basket.insert(String::from("banana"), 7);
    fruit_basket.insert(String::from("orange"), 6);

    let some_fruits = vec!["apple", "banana", "orange", "kiwi", "jack-fruit"];

    for fruit in some_fruits {
        match fruit_basket.get(fruit) {
            Some(val) => println!("{} exist in fruit_basket with a amount of {}", fruit, val),
            None => {
                println!("{} does not exist so we will add it", fruit);
                fruit_basket.insert(String::from(fruit), 1);
            }
        }
    }
    show_basket(&mut fruit_basket);
}
