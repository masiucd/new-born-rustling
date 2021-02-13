// use math::round;
use std::collections::HashMap;

fn zip_to_map(xs1: &Vec<&str>, xs2: &Vec<i32>) {
    let map: HashMap<_, _> = xs1.into_iter().zip(xs2.into_iter()).collect();
    println!("{:?}", map);
}

fn anagram(w1: &str, w2: &str) -> bool {
    let mut map = HashMap::new();

    for c in w1.chars() {
        let c = map.entry(c).or_insert(0);
        *c += 1;
    }

    println!("{:?}", map);
    for c in w2.chars() {
        let foo = map.entry(c).or_default();
        *foo -= 1;
        if map.get(&c).expect("0") < &0 {
            return false;
        }
    }
    true
}

fn mean(xs: &Vec<i32>) -> f32 {
    let len = xs.len() as f32;
    let mut sum: f32 = 0.0;
    for &i in xs.iter() {
        let x = i as f32;
        sum += x;
    }
    sum / len
}

// fn median(xs: &Vec<i32>) -> &i32 {
//     let middle_index = (&xs[0] + &xs[xs.len() - 1]) / 2;

//     // if &xs.len() % 2 != 0 {
//     // } else {
//     // }
// }

fn main() {
    let mut store = HashMap::new();

    let word = "Legia warszawa";

    for c in word.chars() {
        let c = store.entry(c).or_insert(0);
        *c += 1;
    }

    println!("{:?}", store);
}
