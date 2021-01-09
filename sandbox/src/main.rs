enum Clock {
    Semper(u8),
    Analog(u8, u8),
    Digital(u8, u8, u8),
}
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn calc_clock(clock: Clock) {
    match clock {
        Clock::Semper(h) => println!("time is {}", h),
        Clock::Analog(h, m) => println!("time is {} passed {}", h, m),
        Clock::Digital(h, m, s) => println!(" {} {} {} ", h, m, s),
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.01"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home = {:#?}", home);
    calc_clock(Clock::Semper(12))
}
