fn main() {
    let cars = vec!["audi", "mercedes", "bmw", "volvo", "ferrari"];
    let mut xs: Vec<&str> = Vec::new();
    for car in cars.iter() {
        println!("{}", car)
    }
    xs.push("apple");
    xs.push("banana");

    for (i, x) in xs.iter().enumerate() {
        println!("{},{}", i, x)
    }
}
