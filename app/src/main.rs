fn main() {
    let cars: Vec<String> = vec!["audi", "mercedes", "bmw", "volvo", "ferrari"]
        .iter()
        .map(|x| x.to_uppercase())
        .collect();

    println!("{:?}", cars);
}
