#[derive(Debug)]
enum FruitType {
    Round,
    Oval,
    Sour,
    Sweet,
}

#[derive(Debug)]
struct Fruit {
    name: String,
    f_type: FruitType,
    color: String,
    rating: u8,
}

impl Fruit {
    fn eat(self, f_type: FruitType) {
        println!("mmm {:?} am eating typeof {:?}", self.name, f_type);
    }
    fn new(name: String, f_type: FruitType, color: String, rating: u8) -> Fruit {
        Fruit {
            name: name,
            f_type: f_type,
            color: color,
            rating: rating,
        }
    }
}

fn foo(f: FruitType) {
    match f {
        FruitType::Oval => println!("mmmm"),
        FruitType::Round => println!("mmmm,...round"),
        FruitType::Sour => println!("mmmm,sour"),
        FruitType::Sweet => println!("mmmm,ssweet"),
    }
}

fn main() {
    let apple = Fruit {
        name: String::from("granny smith"),
        f_type: FruitType::Sour,
        color: String::from("green"),
        rating: 5,
    };

    let banana = Fruit::new(
        String::from("banana"),
        FruitType::Sweet,
        String::from("yellow"),
        5,
    );

    apple.eat(FruitType::Sour);
    foo(FruitType::Sweet);
    println!("{:?}", banana);
}
