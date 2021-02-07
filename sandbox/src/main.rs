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

fn main() {
    let s1 = "abcd";
    let s2 = "abcd";
    let res = anagram(&s1, &s2);
    println!("{}", res);
}
