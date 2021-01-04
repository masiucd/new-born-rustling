fn main() {
    let xs = ["rust", "js", "go", "haskell"];

    for i in 1..10 {
        println!("{}", i);
    }

    for x in xs.iter() {
        println!("{}", x);
    }
}
