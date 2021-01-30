fn main() {
    let xs: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

    let evenXs = evens(&xs);
    println!("{:?}", evenXs);
}

fn evens(xs: &Vec<i32>) -> Vec<i32> {
    let mut newXs: Vec<i32> = Vec::new();

    for x in xs.iter() {
        if x % 2 == 0 {
            newXs.push(*x);
        }
    }
    newXs
}
