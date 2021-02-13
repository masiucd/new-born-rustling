// use std::fs::File;
// use std::io::ErrorKind;
use fs::File;
use io::Read;
use std::{fs, io};

fn read_username_from_file(file_name: String) -> Result<String, io::Error> {
    let mut s = String::new();

    File::open(file_name)?.read_to_string(&mut s)?;

    Ok(s)
}

fn main() {
    let result = read_username_from_file(String::from("data.json"));

    match result {
        Ok(s) => println!("{}", s),
        Err(e) => println!("err{}", e),
    }

    // can still use result because of Rust shadowing!!!
    let result = read_username_from_file(String::from("file.txt"));

    match result {
        Ok(s) => println!("{}", s),
        Err(e) => println!("err{}", e),
    }

    let s = String::from("hello");
    rev(&s);
}

fn rev(s: &str) -> String {
    let mut res = String::new();

    for i in s.chars() {
        res = i.to_string() + &res;
    }
    println!("{}", res);
    res
}
