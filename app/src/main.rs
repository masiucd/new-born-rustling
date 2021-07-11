fn print_items(xs: &[i32]) {
    for x in xs {
        println!("x is {}", x);
    }
}

fn give_me_vec_slice<'a>(xs: &'a [&str], from: usize, to: usize) -> Vec<&'a str> {
    let parts = &xs[from..to];
    parts.to_vec()
}

fn main() {
    let list: Vec<i32> = (0..20).collect();
    println!("{:?}", list);

    let doubled: Vec<i32> = list.iter().map(|x| x * 2).collect();
    print_items(&doubled);

    let words: Vec<&str> = vec!["the", "quick", "brown", "fox", "jumped"];

    let new_words = give_me_vec_slice(&words, 1, 4);

    println!("{:?}", new_words)
}
