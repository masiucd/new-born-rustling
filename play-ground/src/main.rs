#[derive(Debug)]
struct Dog {
    name: String,
    age: u8,
    owner: Person,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    cool: bool,
}

impl Dog {
    fn new(name: String, age: u8, owner: Person) -> Self {
        Dog { name, age, owner }
    }

    fn greet(&self) {
        println!(
            "Hello my name is {} and my owner is {:?}",
            self.name, self.owner.name
        );
    }
}

impl Person {
    fn new(name: String, age: u8, cool: bool) -> Self {
        Person { name, age, cool }
    }
}

fn main() {
    let bob = Person::new(String::from("Bob"), 34, true);
    let mike = Person::new(String::from("Mike"), 21, true);

    let snickers = Dog::new(String::from("Snickers"), 2, bob);
    let logan = Dog::new(String::from("Logan"), 6, mike);

    snickers.greet();
    // Hello my name is Snickers and my owner is "Bob"
}
