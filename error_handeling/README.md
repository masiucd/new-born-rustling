# Project Title

## Table of Contents

- [About](#about)

## About <a name = "about"></a>

Error handling in `Rust` is different from you are used to if you coming from a `JS`,`Java` ore any `C`-based language. We don't have `null` in `Rust`.
There are different ways and scenarios how to handle errors, sometimes we want to recover from a error and sometimes we need to stop our program with a `panic!` macro.

By panicking in our code we tell `Rust` that the process during our code does not make any sense and we need to stop what we currently doing. You could see it like `throw new Error` if you coming form a `Javascript` background.
Rather then continuing the process with a invalid input it would make more sense to panicking, like press the stop/emergency button.

Let's show a example when a panic would make sense, and we would like to stop our program if something does not make any sense.

```rust
#[derive(Debug)]
enum PlayerPosition {
    Goalie,
    Midfielder,
    Striker,
}

impl PlayerPosition {
    fn parse(pos: &str) -> PlayerPosition {
        if pos == "Goalie" {
            PlayerPosition::Goalie
        } else if pos == "Midfielder" {
            PlayerPosition::Midfielder
        } else if pos == "Striker" {
            PlayerPosition::Striker
        } else {
            panic!("Oppps we could not find no more position {}", pos)
        }
    }
}

fn main() {
    let str = "Goalie";
    let mut pos = PlayerPosition::parse(&str);
    println!("{:?}", pos);
}

```

if we run the example with `Goalie` as a input we will get the expected `Enum` variant.
If we would send something that does not exist on our `Enum` the program will run in a panic, using the `panic!` macro.

`Rust` also provide some other macros that are useful to handle cause a panic.
First we have the `unreachable!` macro. This is useful when to tell the compiler that it is impossible to reach a given point in our program.

for example:

```rust
  enum MoodState {
    Happy,
    Sad,
}

enum MoodAction {
    Smile,
    Cry,
}

fn take_action(current_state: MoodState, action: MoodAction) {
    match (current_state, action) {
        (MoodState::Happy, MoodAction::Smile) => {
            println!("I am happy and I smile")
        }
        (MoodState::Sad, MoodAction::Cry) => {
            println!("I am sad and I am crying")
        }
        _ => unreachable!(),
    }
}

fn main() {
    take_action(MoodState::Happy, MoodAction::Smile);
    take_action(MoodState::Sad, MoodAction::Smile); // will panic with the unreachable! macro
}

```

An `IO` example with error handling

```rust
use fs::File;
use io::Read;
use std::{fs, io};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("file.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
fn main() {
    let s = read_username_from_file();

    match s {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e),
    };
}

```

Ore a more elegant way.

```rust
use fs::File;
use io::Read;
use std::{fs, io};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("file.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
fn main() {
    let result = read_username_from_file();

    match result {
        Ok(s) => println!("{}", s),
        Err(e) => println!("err{}", e),
    }
}

```
