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
}

fn main() {
    let mike = Player::new(String::from("Mike"), String::from("smith"), 32);

    println!("p1 is {:?}", mike.full_name())
}
