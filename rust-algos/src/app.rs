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
}
