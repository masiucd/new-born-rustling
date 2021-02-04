fn fizz_buzz(n: u32) {
    for i in 0..n {
        match i {
            i if (i % 5 == 0 && i % 3 == 0) => println!("fizz buzz"),
            i if (i % 3 == 0) => println!("fizz"),
            i if (i % 5 == 0) => println!("buzz"),
            _ => println!("{}", n),
        }
    }
}

fn main() {
    fizz_buzz(100);
}
