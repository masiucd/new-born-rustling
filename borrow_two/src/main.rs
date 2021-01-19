#[derive(Debug)]
struct Bucket {
    liters: u32,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    dogs: Vec<String>,
}

impl Person {
    fn add_new_dog(&mut self, dog_name: String) {
        self.dogs.push(dog_name);
    }
    fn birthday(&mut self) {
        self.age += 1;
    }
    fn show_dogs(self) {
        for (i, dog) in self.dogs.iter().enumerate() {
            println!("dog number: {} {} ", i, dog);
        }
    }
}

fn pour(source: &mut Bucket, target: &mut Bucket, amount: u32) {
    source.liters -= amount;
    target.liters += amount;
}
fn main() {
    let mut bucket1 = Bucket { liters: 20 };
    let mut bucket2 = Bucket { liters: 14 };

    let mut bob = Person {
        name: "Bob".to_string(),
        age: 21,
        dogs: vec![],
    };

    pour(&mut bucket1, &mut bucket2, 10);

    println!("bucket1 {:?}", bucket1);
    println!("bucket2 {:?}", bucket2);

    println!("person bob's dogs {:?} ", bob.dogs);
    bob.add_new_dog(String::from("Snickers"));
    bob.add_new_dog(String::from("Mia"));
    println!("person bob's dogs {:?} ", bob.dogs);

    bob.birthday();

    println!("Bob had his birthday and is now {:?} years old", bob.age);

    bob.show_dogs();
}
