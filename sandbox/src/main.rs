#[derive(Debug)]
enum Fruit {
    Apple,
    Orange,
    Banana,
}

#[derive(Debug)]
struct User {
    username: String,
    age: u8,
    cool: bool,
    email: String,
    password: String,
}

#[must_use]
impl User {
    fn greet(&self) {
        println!(
            "Hello my name is {} and I am {} years old",
            self.username, self.age
        );
    }
    fn new(user: User) -> User {
        User {
            username: user.username,
            age: user.age,
            cool: user.cool,
            email: user.email,
            password: user.password,
        }
    }
    fn birthday(&self) {
        // &self.age + 1;
        // &self.age + 10
        // &self.age = &self.age + 1;
        println!("{}", &self.age + 1);
    }
}

fn what_fruit(f: Fruit) {
    match f {
        Fruit::Apple => println!("mmm {:?} are so good", f),
        Fruit::Orange => println!("round and juicy {:?}", f),
        Fruit::Banana => println!("yellow and sweet {:?} ", f),
    }
}

fn what_num(x: u32) -> u32 {
    if x > 10 {
        x
    } else {
        x + x
    }
}

fn main() {
    what_fruit(Fruit::Banana);
    let num = what_num(2);
    println!("num is  {} ", num);

    let bobby = User {
        username: String::from("Bobby"),
        email: String::from("b@io.com"),
        cool: true,
        age: 21,
        password: String::from("123456"),
    };

    let mut linda = User::new(User {
        username: String::from("Linda"),
        email: String::from("linda@io.com"),
        cool: true,
        age: 32,
        password: String::from("123456"),
    });

    println!("bobby {:?}", bobby);
    // bobby.greet();
    linda.greet();
    &linda.birthday();
    &linda.greet();
}
