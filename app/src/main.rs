fn count_with_closures() {
    let mut count = 0;

    let mut increment = || {
        count += 1;
        println!("count ={}", count);
    };

    increment();
    increment();
    increment();
    increment();
    increment();
    increment();
}

fn main() {
    count_with_closures();
}
