fn main() {
    let s = String::from("book");

    let res = pluralize(&s);

    println!("I have one {} you have two {} ", s, res);

    let a = String::from("apple");
    let res2 = pluralize2(a.clone());
    println!("I have one {} you have two {} ", a, res2);
}

fn pluralize(s: &String) -> String {
    let mut x = s.clone();
    x.push('s');
    x
}

fn pluralize2(singular: String) -> String {
    singular + "s"
}
