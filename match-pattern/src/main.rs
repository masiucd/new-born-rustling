fn main() {
    let x = 2;

    match x {
        1 => println!("{} is to small", x),
        2 => println!("{} is still is to small", x),
        3 => println!("{} ,is correct", x),
        _ => println!("no match"),
    }

    roll_dice(1, 5)
}

fn roll_dice(die1: u8, die2: u8) {
    match (die1, die2) {
        (1, 1) => println!("Go back!"),
        (5, _) | (_, 5) => {
            println!("You rolled at leas one 5!");
            println!("Move and the roll once again");
        }
        _ => println!("Move your piece!"),
    }
}
