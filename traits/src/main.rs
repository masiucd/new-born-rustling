#[derive(Debug)]
struct Player {
    first_name: String,
    last_name: String,
    age: i32,
}

// almost like a interface if you come from TS
trait FullName {
    fn full_name(&self) -> String;
    fn new(first_name: String, last_name: String, age: i32) -> Player;
    fn birthday(&mut self);
}

impl FullName for Player {
    fn full_name(&self) -> String {
        format!("{},{}", self.first_name, self.last_name)
    }
    fn new(first_name: String, last_name: String, age: i32) -> Player {
        Player {
            first_name,
            last_name,
            age,
        }
    }
    fn birthday(&mut self) {
        self.age += 1;
    }
}

fn main() {
    let mut mike = Player::new(String::from("Mike"), String::from("smith"), 32);
    println!("p1 is {:?}", mike.full_name());
    println!("p1 is {:?} year old", mike.age);
    mike.birthday();
    println!("p1 is now {:?} year old because he has birthday", mike.age);
}
