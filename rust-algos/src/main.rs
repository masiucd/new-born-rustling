// use std::collections::HashMap;

fn main() {
    // let mut res = check_palindrome(String::from("apa"));
    // let r = String::from("III");
    // roman_to_int(r);
    let res = adjacent_elements_product(vec![3, 6, -2, -5, 7, 3]);
    println!("res {}", res);
}

fn adjacent_elements_product(input_array: Vec<i32>) -> i32 {
    let mut res = <i32>::min_value();
    for i in 0..input_array.len() - 1 {
        let prod = input_array[i] * input_array[i + 1];
        if prod > res {
            res = prod;
        }
    }
    res
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
