use std::vec::Vec;

fn iterate_through_list(xs: &[i32]) {
    for x in xs {
        println!("x is {}", x);
    }
}

fn reverse_str(s: &str) -> String {
    let res = s.chars().rev().collect();
    res
}
fn reverse_str2(s: &str) -> String {
    let mut rev_str = String::new();
    for byte in s.chars().rev() {
        rev_str.push(byte);
    }
    rev_str
}

fn string_to_list(string: &String) -> Vec<char> {
    let xs = string.chars().collect::<Vec<char>>();
    for ch in &xs {
        println!("char is {}", ch);
    }
    xs
}

fn main() {
    let a = [1, 2, 3, 4, 5];
    iterate_through_list(&a);
    let name = "marcell";
    let r_name = reverse_str(name);
    let r_name2 = reverse_str2(name);
    println!("r_name is {}", r_name);
    println!("r_name2 is {}", r_name2);

    let names = vec!["bob", "mike", "greg"];
    match_name(names);

    let hello_world = String::from("Hello world");

    let xs = string_to_list(&hello_world);

    println!("xs is {:?} ", xs);
}

fn match_name(names: Vec<&str>) {
    for name in names.iter() {
        match name {
            &"bob" => println!("{} is cool", name),
            &"mike" => println!("{} like to go on holidays", name),
            &"greg" => println!("{} is chill", name),
            _ => println!("no match"),
        }
    }
}
