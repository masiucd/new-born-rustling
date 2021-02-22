#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    cool: bool,
}

impl Person {
    fn new(name: String, age: u32, cool: bool) -> Person {
        Person { name, age, cool }
    }

    fn birth_day(&mut self) {
        self.age += 1
    }

    fn new_name(&mut self, name: String) {
        self.name = name
    }
}

fn main() {
    let mut user1 = Person::new("John".to_string(), 21, true);
    user1.birth_day();
    user1.new_name("Frank".to_string());
    println!("{:?}", user1);
}
