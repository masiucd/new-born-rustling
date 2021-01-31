use std::collections::HashMap;

fn count_chars(s: &str) {
  let mut freqs = HashMap::new();
  let one_string: String = s.split_whitespace().collect();

  for char in one_string.chars() {
    *freqs.entry(char).or_insert(0) += 1;
  }

  println!("freqs is now {:?}", freqs);
}

#[derive(Debug)]
struct Stats {
  hp: u8,
  sp: u8,
}

#[derive(Debug)]
struct Monster {
  friends: Vec<Friend>,
  stats: Stats,
}

#[derive(Debug)]
struct Friend {
  loyalty: u8,
}

impl Stats {
  fn new(hp: u8, sp: u8) -> Self {
    Stats { hp: hp, sp: sp }
  }
  fn heal(&mut self, life: u8) {
    self.hp += life;
    self.sp += life;
  }
}

impl Monster {
  fn new(hp: u8, sp: u8) -> Self {
    Monster {
      friends: vec![],
      stats: Stats::new(hp, sp),
    }
  }

  fn add_friend(&mut self, friend: Friend) {
    self.friends.push(friend);
  }

  fn final_berate(&mut self) {
    if let Some(friend) = self.friends.first() {
      self.stats.heal(friend.loyalty);
      println!("Healing for {}", friend.loyalty);
    }
  }
}

impl Friend {
  fn new(loyalty: u8) -> Self {
    Friend { loyalty: loyalty }
  }
}

fn main() {
  // let sen = "Hello world!";
  // count_chars(&sen);

  let friend1 = Friend::new(10);
  let mut monster1 = Monster::new(100, 80);
  monster1.add_friend(friend1);
  monster1.final_berate();

  println!("{:?}", monster1);
}
