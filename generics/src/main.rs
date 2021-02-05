#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct User<T> {
    name: String,
    last_name: String,
    age: T,
    cool: bool,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }

    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }
}

impl Point<i32> {
    fn sum_x_and_y(&self) -> i32 {
        &self.x + &self.y
    }
}

impl User<i32> {
    fn new(name: String, last_name: String, age: i32, cool: bool) -> User<i32> {
        User {
            name,
            last_name,
            age,
            cool,
        }
    }

    fn full_name(&self) -> String {
        format!("{} {} ", self.name, self.last_name)
    }
}

fn get_user(name: &str) -> Option<&str> {
    // if we find the user
    if name == "foo" {
        Some(name)
    } else {
        None
    }
}

fn main() {
    let p1 = Point::new(20, 32);
    println!("{}", p1.sum_x_and_y());
    // let u1 = get_user("bob");
    // println!("{:?}", u1);

    // let u2 = get_user("foo");
    // println!("{:?}", u2);

    // let u1:User<i32> = User::new(String::from("greg"), age: 12, cool: true);
}
