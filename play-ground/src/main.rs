#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    has_girl: Option<bool>,
}

impl Person {
    fn new(name: String, age: u8, has_girl: Option<bool>) -> Person {
        Person {
            name,
            age,
            has_girl,
        }
    }
    fn has_girl_friend(self) {
        match self.has_girl {
            Some(b) => println!("has a girl friend? {}", b),
            None => println!("Opps"),
        }
    }
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

struct Pair(i32, i32);

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
    fn multiply(&self) -> f32 {
        // destructuring with pattern match
        match self {
            Point { x, y } => x * y,
        }
    }
}

impl Rectangle {
    fn new(top_left: Point, bottom_right: Point) -> Rectangle {
        Rectangle {
            top_left,
            bottom_right,
        }
    }

    fn area(&self) -> f32 {
        // (self.top_left.x + self.top_left.y) * (self.bottom_right.x + self.bottom_right.y)
        &self.top_left.multiply() + &self.bottom_right.multiply()
    }
}

fn main() {
    let pete = Person::new(String::from("Pete"), 26, Some(false));

    println!("{:?}", pete.has_girl.expect("ooops"));
    pete.has_girl_friend();

    let p1 = Point::new(45.5, 55.9);
    let p2 = Point::new(45.5, p1.y);

    let rec = Rectangle::new(p1, p2);

    let rec_area = rec.area();
    println!("{}", rec_area);

    let ppp = Point::new(1.2, 2.2);

    // simple destructuring
    let Point { x, y } = ppp;
    println!("{},{}", x, y);
}
