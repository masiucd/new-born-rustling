fn main() {
    let mut a = vec![1, 2, 3, 4, 5];

    let last = &a.pop().expect("ooopa");

    // match last {
    //     Some(l) => println!("{}", l),
    //     None => println!("nope!"),
    // }

    println!("{}", last);
    println!("{:?}", a);
}
