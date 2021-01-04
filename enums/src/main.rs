#[derive(Debug)]

enum FootBallPlayer {
    GoalKeeper,
    Defender,
    Midfielder,
    Attacker,
}

enum Clock {
    Sundial(u8),
    Digital(u8, u8),
    Analog(u8, u8, u8),
}

enum ChocolateSquares {
    Two(u8, u8),
    Four(u8, u8, u8, u8),
    Six(u8, u8, u8, u8, u8, u8),
}

fn munch(c: ChocolateSquares) {
    match c {
        ChocolateSquares::Two(a, b) => println!("I am eating {},{} ", a, b),
        ChocolateSquares::Four(a, b, c, d) => println!("{},{},{},{}", a, b, c, d),
        ChocolateSquares::Six(a, b, c, d, e, f) => println!("{},{},{},{},{},{} ", a, b, c, d, e, f),
    }
}

fn greet(p: FootBallPlayer) {
    println!("hello {:?} ", p)
}

fn tell_time(clock: Clock) {
    match clock {
        Clock::Sundial(hours) => println!("is about {} o'clock", hours),
        Clock::Digital(hours, min) => println!("it is {} minutes past {}", min, hours),
        Clock::Analog(h, m, s) => println!("It is {} min and {} sec past {} o'clock", h, m, s),
    }
}

fn main() {
    let buffon = FootBallPlayer::GoalKeeper;
    let ferdinand = FootBallPlayer::Defender;
    let schools = FootBallPlayer::Midfielder;
    let ronaldo = FootBallPlayer::Attacker;

    greet(buffon);

    tell_time(Clock::Analog(2, 1, 2));
    tell_time(Clock::Sundial(12));
    tell_time(Clock::Digital(11, 5));

    munch(ChocolateSquares::Six(1, 2, 6, 7, 4, 3))
}
