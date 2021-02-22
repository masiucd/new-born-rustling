// fn highest_num<T>(xs: &[T]) -> &i32 {
//     let mut max = &xs[0];

//     for i in xs.iter() {
//         if max < i {
//             max = i
//         }
//     }

//     max
// }

#[derive(Debug)]
struct Point<T> {
    y: T,
    x: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
    fn mix_up(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p1 = Point::new(45.2, 22.21);
    let p2 = Point::new(49.4, 12.31);

    println!("{:?}", p1.distance_from_origin());

    let x = p1.mix_up(p2);
    println!("{:?}", x);

    let integer = Some(99);

    // println!("{:?}", integer);

    match integer {
        Some(integer) => println!("{}", integer),
        None => println!("no int"),
    }

    // let xs = vec![1, 99, 2, 3, 66, 4];
    // let chars = vec!['a', 'b', 'c', 'd', 'e'];

    // let res = highest_num::<i32>(&xs);
    // let res = highest_num::<char>(&chars);
    // println!("{}", res);
}
