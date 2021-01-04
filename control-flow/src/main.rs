use std::io;

fn main() {
    discount(10);
    let r = foo(30);
    println!("{}", r);
    // who_is_the_master()
    // who_is_the_master2()
}

fn foo(n: u8) -> u8 {
    if n > 10 {
        n / 2
    } else {
        n * 2
    }
}

fn discount(day_of_month: u8) {
    let amount = if day_of_month % 2 == 0 { 50 } else { 10 };

    println!("Your discount is {}%", amount);
}

fn who_is_the_master() {
    loop {
        println!("who is the master?");
        let mut master = String::new();
        io::stdin()
            .read_line(&mut master)
            .expect("Failed to read line");

        if master.trim() == "masiu" {
            break;
        }
    }

    println!("My man!");
}

fn who_is_the_master2() {
    let mut master = String::new();

    while master.trim() != "masiu" {
        println!("Who is the master?");
        master = String::new();
        io::stdin()
            .read_line(&mut master)
            .expect("Failed to read line");
    }

    println!("My Man!");
}
