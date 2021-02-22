fn main() {
    let list = vec![1, 2, 3, 4, 5];
    // Longer lifetime
    {
        // shorter-lifetime
        let list_two = &list[0..4];
        println!("{:?}", list_two);
    }
    println!("{:?}", list);
}
