fn main() {
    let xs = vec![1, 2, 3, 4, 5];

    let mapped = xs.iter().map(|x| x * 2).collect::<Vec<i32>>();

    println!("xs= {:?}", xs);
    println!("mapped= {:?}", mapped);
}
