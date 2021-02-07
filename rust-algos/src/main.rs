// use std::collections::HashMap;
mod app;

fn main() {
    //  app::algos::adjacent_elements_product(vec![3, 6, -2, -5, 7, 3]);
    // println!(
    //     "is_palindrome = {}",
    //     app::algos::is_palindrome(String::from("racecar"))
    // );

    // println!("{}", app::algos::search_insert(vec![1, 2, 3, 5, 6], 4))
    // app::algos::fizz_buzz(100);

    let res = app::algos::is_anagram(String::from("anagram"), String::from("anagrma"));
    println!("{}", res);
}
