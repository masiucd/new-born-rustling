#[derive(Debug)]
enum Temp {
    Celsius,
    Fahrenheit,
}

fn celsius_to_fahrenheit(c: i32) -> i32 {
    let r = c * 9 / 5 + 32;
    r
}

fn fahrenheit_to_celsius(c: i32) -> i32 {
    (c - 32) * 5 / 9
}

fn fib(n: u32) -> u32 {
    if n <= 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn calculate(t: Temp, c: i32) {
    match t {
        Temp::Celsius => println!(
            "{} celsius is = {} fahrenheit ",
            c,
            celsius_to_fahrenheit(c)
        ),
        Temp::Fahrenheit => println!(
            "{} fahrenheit is = {} celsius ",
            c,
            fahrenheit_to_celsius(c)
        ),
    }
}

fn main() {
    calculate(Temp::Celsius, 20);
    calculate(Temp::Fahrenheit, 10);

    let f = fib(5);
    println!("{}", f);
}
