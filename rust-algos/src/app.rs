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
  pub fn mean(xs: &Vec<i32>) -> f32 {
    let len = xs.len() as f32;
    let mut sum: f32 = 0.0;
    for &i in xs.iter() {
      let x = i as f32;
      sum += x;
    }
    sum / len
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

  pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
      return false;
    }

    let mut char_count1 = std::collections::HashMap::with_capacity(s.len());
    let mut char_count2 = std::collections::HashMap::with_capacity(t.len());

    for char in s.chars() {
      *char_count1.entry(char).or_insert(0) += 1
    }

    for char in t.chars() {
      *char_count2.entry(char).or_insert(0) += 1
    }

    return char_count1 == char_count2;
  }

  pub fn reverse_str_3(s: &str) -> String {
    let mut res = String::new();

    for i in s.chars() {
      res = i.to_string() + &res;
    }
    println!("{}", res);
    res
  }
}
