#[derive(Debug)]
enum Clock {
    A(u8),
    B(u8, u8),
    C(u8, u8, u8),
}

#[derive(Debug)]
struct Thing {
    name: String,
    clock: Clock,
}

impl Thing {
    fn new(name: String, clock: Clock) -> Thing {
        Thing { name, clock }
    }
    fn match_clock(&self) {
        match self.clock {
            Clock::A(a) => println!("just hours of {}", a),
            Clock::B(a, b) => println!("just hours {} and minutes {}", a, b),
            Clock::C(a, b, c) => println!("just hours {}  minutes {} and seconds {}", a, b, c),
        }
    }
}

fn main() {
    let thing_one = Thing::new("foo".to_string(), Clock::B(21, 2));
    println!("{:?}", thing_one);
    thing_one.match_clock()
}
