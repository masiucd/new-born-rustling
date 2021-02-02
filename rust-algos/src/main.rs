fn main() {
    let mut res = check_palindrome(String::from("apa"));
    println!("{}", res);
}

fn check_palindrome(inputString: String) -> bool {
    inputString.chars().rev().collect::<String>() == inputString
}
