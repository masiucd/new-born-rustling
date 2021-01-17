fn main() {
    let name = "marcell";
    let name_slice = &name[0..3];
    let cell = &name[3..];
    let barrow_all_data = &name[..];
    println!(
        "name_slice is {} and the length is {:?} ",
        name_slice,
        name_slice.chars().count()
    );

    println!("{}", cell);
    println!("{}", barrow_all_data);

    let nums: [i32; 3] = [1, 2, 3];
    let num_slice = &nums[0..3];
    let mut vec_nums = vec![99, 100, 12];
    vec_nums.push(2);
    println!("{:#?}", vec_nums);

    println!("{:#?}", num_slice);

    let words = vec!["the", "quick", "brown", "fox", "jumped"];
    let w = &words[1..4];
    println!("{:#?}", w);

    only_reference_to_array(&nums);
    only_reference_to_vector(&vec_nums);
    reference_to_either_arrray_or_vector(&vec_nums);
    reference_to_either_arrray_or_vector(&nums);
    print_list_slice(&nums);

    let vector_list = [1, 2, 3, 4, 5, 6];

    double(&vector_list);
}

fn only_reference_to_vector(v: &Vec<i32>) {
    println!("this is a vector {:?}", v);
}
fn only_reference_to_array(a: &[i32; 3]) {
    println!("this is a array {:?}", a);
}
fn reference_to_either_arrray_or_vector(xs: &[i32]) {
    println!("this is a slice {:?}", xs);
}

fn print_list_slice(xs: &[i32]) {
    for x in xs {
        println!("x is {}", x);
    }
}

fn double(xs: &[i32]) {
    let res = xs.iter().map(|x| x * 2).collect::<Vec<i32>>();
    println!("{:?}", res);
}
