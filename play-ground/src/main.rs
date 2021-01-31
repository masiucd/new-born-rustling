fn main() {
    let x = 0u8;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => println!("no match"),
    }
}
