// use std::collections::HashMap;
mod app;

fn main() {
    //  app::algos::adjacent_elements_product(vec![3, 6, -2, -5, 7, 3]);
    println!(
        "is_palindrome = {}",
        app::algos::is_palindrome(String::from("racecar"))
    );

    println!("{}", app::algos::search_insert(vec![1, 2, 3, 5, 6], 4))
}

// fn roman_to_int(s: String) -> i32 {
//     let m: HashMap<char, i32> = [
//         ('I', 1),
//         ('V', 5),
//         ('X', 10),
//         ('L', 50),
//         ('C', 100),
//         ('D', 500),
//         ('M', 1000),
//     ]
//     .iter()
//     .cloned()
//     .collect();

//     let xs: Vec<char> = s.chars().collect();
//     for &char in &xs {
//         // println!("{:?}", m.get(&char));
//         match m.get(&char) {
//             Some(value) => println!("key={}, value={}", char, value),
//             None => println!("noo {} ", char),
//         }
//     }

//     0
// }

// fn check_palindrome(input_string: String) -> bool {
//     input_string.chars().rev().collect::<String>() == input_string
// }
