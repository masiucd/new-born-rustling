pub mod algos {

  pub fn adjacent_elements_product(input_array: Vec<i32>) -> i32 {
    let mut res = <i32>::min_value();
    for i in 0..input_array.len() - 1 {
      let prod = input_array[i] * input_array[i + 1];
      if prod > res {
        res = prod;
      }
    }
    res
  }

  pub fn is_palindrome(s: String) -> bool {
    s.chars().rev().collect::<String>() == s
  }

  pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    for (i, v) in nums.iter().enumerate() {
      if target < *v {
        return i as i32;
      }
    }
    return nums.len() as i32;
  }

  pub fn fizz_buzz(n: i32) {
    for i in 0..n {
      match i {
        i if (i % 5 == 0) => println!("Fizz"),
        i if (i % 3 == 0) => println!("Buzz"),
        i if (i % 15 == 0) => println!("Fizz Buzz"),
        _ => println!("{}", i),
      }
    }
  }
}
