# New born ruslang

## Table of Contents

- [New born ruslang](#new-born-ruslang)
  - [Table of Contents](#table-of-contents)
  - [About](#about)
  - [Data types](#data-types)
  - [Functions](#functions)
  - [Statement vs Expression](#statement-vs-expression)
  - [Tuples](#tuples)
  - [Control Flow](#control-flow)
    - [guising game](#guising-game)
  - [Enums](#enums)
  - [Structs](#structs)
  - [Methods](#methods)
  - [Associated functions](#associated-functions)
  - [Borrowing](#borrowing)
  - [Ownership <a name ="ownership"></a>](#ownership-)
  - [Borrowing Patterns <a name = "#borrowing-patterns"> </a>](#borrowing-patterns--)
  - [Slices <a name ="slices"></a>](#slices-)
  - [Self <a href = "self"></a>](#self-)
  - [Result and Options <a name="rao"></a>](#result-and-options-)
  - [Generics <a name ="generics"></a>](#generics-)
  - [Functional programming <a name ="functional programming"></a>](#functional-programming-)
      - [Playground](#playground)
      - [Higher order functions in `Rust`](#higher-order-functions-in-rust)

## [About](#about)

New born rustlang, a beginner repo that will go through the very basics in rust such as
`[Control Flow](#cf)`, `[Structs](#structs)` `[Methods](#methods)` and so much more.

Rust is a system level programing language, but it is so much more thane that. Me personally that coming from a `Javascript` background, Rust is a game changer, people think that web-assembly and rust will replace `Javascript` but I don't think it is like that. I know that Rust together with Javascript will make `Javascript` so much more powerful and only the feature can tell us what it will bring.

Javascript developers will use Rust and web assembly without even knowing it, if you don't want of course, for example front end frameworks like `React` or `Svelte` could be even more optimized with `Rust` and thats is what I mean that you will probably use it without even knowing it. The thing you will make notice of is how much faster and precise everything would be.

This Repo is for you as a `JS` developer to learn the fundamentals in `Rust` and how it could help you to become a much better `Javascript` developer

## [Data types](#data-types)

| Signed | Unsigned |
| ------ | -------- |
| i8     | u8       |
| i16    | u16      |
| i32    | u32      |
| i64    | u64      |

An `i8` will store 8 bits in memory. `I` means signed, and we can store either positive or negative numbers with signed integers.
The rest of the bits store the value of the number. This means that an i8 can store the values between -128 and 127, inclusive.
If we know that the number we will use is just positive we could use a unsigned integer instead, with `u` keyword in front.
Because the unsigned types don't use a bit to store the sign, the range of values they can store is different.
A `u8` can store the values between 0 and 255.
The `isize` and `usize` integer types are architecture dependent.
If you are using a 32-bit machine, they will take up `32-bit` of space.
Using a `64-bit` machine they will take up `64-bit` of space.

We use `isize` and `usize` for indexing into collections or counting items.

```rust
  let num:u32 = 100;
```

Floating point numbers in `Rust` is either a `f32` or `f64` type.
using 32 and 64 bits, respectively.

```rust
  let my_float:f32 = 25.05;
```

Characters are our last simple data type. Rust can store single characters as the char data type.
`char` can hold more than just `ASCII` values since `char` is a Unicode scalar value.
We write characters with single quotes;

```rust
  let my_char = 'a';
  let soda = '🥤';
```

```rust
fn match_tup(tup: &(i32, &str, char, bool)) {
    match tup {
        (1, _, _, _) => println!("it is a integer"),
        (_, _, _, true) => println!("It is true"),
        _ => println!("Either a string ore a char"),
    }
}

fn main() {
    let tup = (1, "apples", '🚀', true);

    match_tup(&tup);
}
```

## [Functions](#functions)

Functions in `Rust` are defined wit the `fn` keyword.
when returning from the function we need to define the type.
If it is not returning anything then we leave the return type.

```rust
fn square(num: i32) -> i32 {
    num * num
}

fn greet(name: &str) {
    println!("Hello {} ", name);
}

fn main() {
    let num = square(10);
    println!("{}", num);
    greet("Mia");
}

```

## [Statement vs Expression](#s-vs-e)

To know statement vs expression is important specially in `rust` when to add
semicolon or not. A statement is something that we currently do in part of code
without returning for example.

```rust
 if a > 10 {
   println!("foo");
 }else{
   println!("bar");
 }
```

as you see wee add semicolon after the `println!` function do declare that this
is a statement and not a expression.

To show ho a expression would look like we could been writing it like this.

```rust
  fn double(x:u8) -> u:8 {
    // also a expression here
    x * 2
  }
  let a:u8 = 20;

  if a > 10 {
    double(a)
  }else{
  // else just return a
   a
  }

```

## [Tuples](#tuples)

A tuple is a collection of different types wrapped in parentheses `()`. There is no fixed value for the tuple but is is common to not have it to long, around 2-4 types i would say `(t1,tw,t3,t4)`.

```rust
  struct TupleType(u8, f32, String, bool);
  let foo = TupleType(2, 2.5, "legia".to_string(), true);
```

```rust
fn main() {
    let matrix_t = ((10, 20), (1, 2));
    println!("{:?}", matrix_t);
    let revesed_tuple_matrix = rev(matrix_t);
}

fn rev(t: ((i32, i32), (i32, i32))) -> ((i32, i32), (i32, i32)) {
    let (t1, t2) = t;
    ((t1.1, t1.0), (t2.1, t2.0))
}

fn reverse_with_tuple(pair: (i32, i32)) -> (i32, i32) {
    let (a, b) = pair;
    (b, a)
}

```

To access a value from the tuple we use dot notation and the spot they are in, starting from 0.

```rust
let foo = TupleType(2, 2.5, "legia".to_string(), true);

println!("{:?}", foo.0); // 2
println!("{:?}", foo.2); // Legia
```

---

## [Control Flow](#control-flow)

Like with functions, we can get a value from an if or else block by leaving the semicolon off of the last expression in the block.
We can then put that value in a variable. Note, that the semicolon is required to end the variable assignment statement.
It is similar to a ternary operator in languages like `Java` or `Javascript` .

```rust
fn main() {
    discount(2);
}

fn discount(day_of_month: u8) {
    let amount = if day_of_month % 2 == 0 { 25 } else { 20 };

    println!("Your discount is {}", amount);
}

```

To create a simple loop in `Rust` we can use the `loop` keyword.
This will give us a infinite loop if don't have any statement in our loop.

```rust
fn main() {
    let mut c = 0;
    loop {
        if c == 10 {
            break;
        }
        c += 1;
        println!("nooooooo");
    }
}
```

### guising game

```rust
use std::io;

fn main() {
    loop {
        println!("best team in the universe?");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        if answer.trim() == "legia" {
            break;
        }
    }
    println!("Great job!");
}

```

The `loop` keyword is similar to a `while` loop and we could also in ths case been using a while loop instead.

```rust
use std::io;

fn main() {
    let mut answer = String::new();
    while answer.trim() != "legia" {
        println!("best team in the universe?");
        answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
    }
    println!("Great job!");
}

```

A simple for loop in `rust`

```rust
for i in 0..11 {
        println!("{}", i)
    }
```

loop over vectors with a index:

```rust
fn main() {
    let cars = vec!["audi", "mercedes", "bmw", "volvo", "ferrari"];
    let mut xs: Vec<&str> = Vec::new();
    for car in cars.iter() {
        println!("{}", car)
    }
    xs.push("apple");
    xs.push("banana");

    for (i, x) in xs.iter().enumerate() {
        println!("{},{}", i, x)
    }
}

```

## [Enums](#enums)

`Enums` allow you to define a type by enumerating its possible variants.
It is very common to use `enums` for pattern matching in `rust`.
`Enums` are great for defining types that can be one of a possible set of values. They can optionally hold extra data, and we can use match expressions to make decisions based on which variant we have, and use the patterns to destructor the values in the variants.
`Rust’s` enums are most similar to algebraic data types in functional programming languages, such as F#, OCaml, and Haskell.
An `Enum` is a a single type. That are fixed and describes a pattern, for example:

```rust
enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
}

```

We access the `variants` from the `enum` via `::` ex `Day::Friday`
An `Enum` can also hold data like tuples,unit struts ore records.

```rust

enum StatusMessage {
    Success,                                // unit variant
    Warning { code: i32, message: String }, // Struct variant
    Error(String),                          // tuple variant
}

fn main() {
    let mut form_status = StatusMessage::Success;
    print_status(form_status);
    form_status = StatusMessage::Warning {
        code: 404,
        message: String::from("Oooops!"),
    };
    print_status(form_status);
    form_status = StatusMessage::Error(String::from("Error!"));
    print_status(form_status);
}

fn print_status(status: StatusMessage) {
    match status {
        StatusMessage::Success => println!("Success "),
        StatusMessage::Warning { code, message } => println!("Warning {} - {}", code, message),
        StatusMessage::Error(msg) => println!("Error: {} ", msg),
    }
}

}
```

an `Enum` variant can also takes some values, and if we would patter match on this the values will be used in the match as well. Notice that in the match we could name them whatever we want.

```rust
#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
struct IpAddress {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let one = IpAddrKind::V4(127, 0, 0, 1);
    let two = IpAddrKind::V6(String::from("::6"));

    show_ip(&one);
    show_ip(&two);
}

fn show_ip(ip: &IpAddrKind) {
    match ip {
        IpAddrKind::V4(a, b, c, d) => println!("a={},b={},c={},d={}", a, b, c, d),
        IpAddrKind::V6(x) => println!("V6 = {}", x),
    }
}

```

```rust
#[derive(Debug)]

enum Mood {
    Sad(String),
    Happy(String),
    Angry(String),
    Bored,
    Nothing,
}

fn main() {
    let mut mood = Mood::Sad(String::from("I am so Sad"));

    show_mood(&mood);

    mood = Mood::Happy(String::from("I want to cry!!!!🥲"));
    show_mood(&mood);

    mood = Mood::Angry(String::from("I just want to punch something😡"));
    show_mood(&mood);

    mood = Mood::Borred;
    show_mood(&mood);

    mood = Mood::Nothing;
    show_mood(&mood);
}

fn show_mood(mood: &Mood) {
    match mood {
        Mood::Sad(msg) => println!("When is sad = {}", msg),
        Mood::Happy(msg) => println!("When is happy = {}", msg),
        Mood::Angry(msg) => println!("When is angry = {}", msg),
        Mood::Bored => println!("😑"),
        Mood::Nothing => println!("∞"),
    }
}

```

One important `Enum` in `Rust` is the built in `Option` enum.

```rust
  enum Option<T> {
    Some(T),
    None,
}
```

There is no null in `Rust` like in many other programing languages, instead to check if something exists we use the `Option` enum.

```rust
  let some_num = Some(10); // Some(10)
  let some_string = Some("Yoo"); // Some("Yoo")
  let none_number: Option<i32> = None; // None

```

If we use None rather than Some, we need to tell `Rust` what type of Option<T> we have,
this because the compiler will not know what type that the `Some` variant will be, only by looking on the `None` value.

How we can match on the `Option<T>` `enum`.

```rust
  fn double(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x * 2),
        None => None,
    }
}

  fn main() {
      let twenty = double(Some(10));
      let non_double = double(None);

      println!("{:?}", twenty); // Some(20)
      println!("{:?}", non_double); // None
  }

```

if we not catch all cases in `Rust` we will get an compile error, to solve this we use the special pattern `_`

```rust
fn main() {
  let x = 0u8;

  match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    _ => println!("no match"),
  }
}
```

```rust
#[derive(Debug)]
enum Clock {
    Sundial(u8),
    Digital(u8, u8),
    Analog(u8, u8, u8),
}

fn main() {
    let mut res = what_clock(Clock::Sundial(10));
    res = what_clock(Clock::Digital(10, 22));
    res = what_clock(Clock::Analog(10, 02, 11));

    println!("{:?}", res);
}

fn what_clock(clock: Clock) {
    match clock {
        Clock::Sundial(hours) => println!("It is about {} hours", hours),
        Clock::Digital(hours, minutes) => println!("It is about {}:{}", hours, minutes),
        Clock::Analog(hours, minutes, seconds) => {
            println!("It is about {}:{}:{}", hours, minutes, seconds)
        }
    }
}

```

---

## [Structs](#structs)

With `Structs` we can structure our `rust` programs in a more logical and proper way.
Structs are used to encapsulate related properties into one unified data type.
`rust` follows a convention to use `PascalCase` when creating structs.

There are 3 different structs in `rust`

1. **C-like structs**

- Similar to classes if you coming from a `OOP` background.
- Fields has key's so we can access properties via dot notation

2. **Tuple structs**

- One ore more values separated with a comma, no key value pairs!
- uses parenthesizes instead of curly brackets

3. **Unit structs**

- useful when using generics

**C-like-struct**

```rust
  #[derive(Debug)]
struct Dog {
    name: String,
    age: u8,
    owner: Person,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    cool: bool,
}

impl Dog {
    fn new(name: String, age: u8, owner: Person) -> Self {
        Dog { name, age, owner }
    }

    fn greet(&self) {
        println!(
            "Hello my name is {} and my owner is {:?}",
            self.name, self.owner.name
        );
    }
}

impl Person {
    fn new(name: String, age: u8, cool: bool) -> Self {
        Person { name, age, cool }
    }
}

fn main() {
    let bob = Person::new(String::from("Bob"), 34, true);
    let mike = Person::new(String::from("Mike"), 21, true);

    let snickers = Dog::new(String::from("Snickers"), 2, bob);
    let logan = Dog::new(String::from("Logan"), 6, mike);

    snickers.greet();
    // Hello my name is Snickers and my owner is "Bob"
}

```

another good example on using `C-like structs`

```rust
#[derive(Debug)]
struct Shape {
    width: u8,
    height: u8,
}

impl Shape {
    fn area(&self) -> u8 {
        self.width * self.height
    }

    fn rectangle(width: u8, height: u8) -> Shape {
        Shape { width, height }
    }

    fn square(side: u8) -> Shape {
        Shape {
            width: side,
            height: side,
        }
    }
}
fn does_fit(s1: &Shape, s2: &Shape) -> bool {
    s1.width >= s2.width && s1.height >= s2.height
}

fn main() {
    let rec = Shape::rectangle(30, 25);
    let square = Shape::square(15);

    println!(
        "this is a square {:?} and this is an rectangle {:?}",
        square, rec
    );

    let does_fit_check_first = does_fit(&rec, &square);
    let does_fit_check_second = does_fit(&square, &rec);

    println!(
        "first check = {}, second check = {} ",
        does_fit_check_first, does_fit_check_second
    );
}

```

A tuple struct has only one element,
we usually call it new-type pattern. Because it helps to create a new type.

```rust
  struct Rgb(u8, u8, u8);
struct IsCool(bool);


fn main() {
    let blue = Rgb(2, 136, 209);
    let cool = IsCool(true);
}

```

Unit structs is not so common to use but are more powerful when combined with other features.

```rust
#[derive(Debug)]
struct Foo;

fn main() {
    let f = Foo;
    println!("{:?}", f); // Foo
}

```

```rust
#[derive(Debug)]
enum FootBallPosition {
    Goalie,
    Defender,
    Midfielder,
    Attacker,
}

#[derive(Debug)]
struct FootBallPlayer {
    name: String,
    number: u8,
    position: FootBallPosition,
}

fn main() {
    let buffon = FootBallPlayer {
        name: String::from("Buffon"),
        number: 1,
        position: FootBallPosition::Goalie,
    };

    let ronaldo = FootBallPlayer {
        name: String::from("Ronaldo"),
        number: 7,
        position: FootBallPosition::Midfielder,
    };

    let football_players: Vec<FootBallPlayer> = vec![buffon, ronaldo];

    for player in football_players {
        println!(
            "players name {}, plays with number {} and is an {:?}",
            player.name, player.number, player.position
        );
    }
}

```

## [Methods](#methods)

**Why Methods?**
We could simply make function that are not bind to out structs and it would work. However To make the code more organized and easier for you teammate to read and understand what's going on, we only want for example football player to be able to score
the goal. It would not make any sense for a `maskot` or a `car` to score a goal right?

```rust
#[derive(Debug)]
enum FootBallPosition {
    Goalie,
    Defender,
    Midfielder,
    Attacker,
}

#[derive(Debug)]
struct FootBallPlayer {
    name: String,
    number: u8,
    position: FootBallPosition,
}

impl FootBallPlayer {
    fn score(&self, time_left_remain: u16) {
        if time_left_remain < 5 {
            println!("miss")
        } else {
            match self.position {
                FootBallPosition::Midfielder => println!(
                    "{} scored and only {:?} can score",
                    self.name, self.position
                ),
                _ => println!("miss"),
            }
        }
    }
}

#[derive(Debug)]
struct Triangle(u32, u32, u32);

fn main() {
    let buffon = FootBallPlayer {
        name: String::from("Buffon"),
        number: 1,
        position: FootBallPosition::Goalie,
    };

    let ronaldo = FootBallPlayer {
        name: String::from("Ronaldo"),
        number: 7,
        position: FootBallPosition::Midfielder,
    };

    let football_players = vec![&buffon, &ronaldo];

    for player in football_players {
        println!(
            "players name {}, plays with number {} and is an {:?}",
            player.name, player.number, player.position
        );
    }

    let check_has_scored = &ronaldo.score(50);
    let check_has_scored2 = &buffon.score(50);
}

```

## [Associated functions](#associated-fns)

`Associated functions` are defined within an impl block on a struct or an enum, the same as methods. They're still are just related to our type, so we want to organize that behavior so that it's associated with the type. Unlike methods, they don't take self as a parameter. Associated functions are commonly used to create instances of types, so they don't have a self instance to operate on.
Lets say we want to create a new instance of our struct, so we create a new function that will create in this case our new dog.

similar like we do when we create a new instance of a class in languages like `Java`.

```java
  SomeClass someClass = new SomeClass();
```

```rust
struct Dog {
  name: String,
  age: i32,
}

impl Dog {
  fn new(name: String,age: i32) -> Dog {
    Dog{ name,age}
  }
}

 Dog::new("snickers", 32);

```

```rust
impl FootBallPlayer {
    fn new(name: String, number: u8, position: FootBallPosition) -> FootBallPlayer {
        FootBallPlayer {
            name,
            number,
            position,
        }
    }
}
```

## [Borrowing](#borrowing)

Borrowing in `Rust` helps us to barrow a the data for a short amount of time, for exempla when passing a user struct into a greet method like this.

```rust
  struct User {
    username: String,
    age: u8,
    cool: bool,
    email: String,
    password: String,
}

impl User {
  fn greet(&self) {
    println!(
      "Hello my name is {} and I am {} years old",
            self.username, self.age
        );
    }
  }

  fn main() {
    let  linda = User::new(User {
      username: String::from("Linda"),
        email: String::from("linda@io.com"),
        cool: true,
        age: 32,
        password: String::from("123456"),
    });
    &linda.greet();
}
```

Borrowing a value may feel similar to pointers if you been working with `C` ore other languages that using pointers.Borrowing is a way to use some code for a time without taking the ownership.
Like someone want's to borrow your car for a while.
Even if your friend is using your car it is still your car and your responsibility to pay the taxes and stuff, in this case when it come to `rust` clean up after you.

```rust
enum AnimalType {
  Dog,
    Cat,
}

struct Animal {
  name: String,
    animalType: AnimalType,
}

// Barrow the animal by making a reference
fn greet(animal: &Animal) {
  println!("hello there {} ", animal.name);
}

fn main() {
    let dog = Animal {
        name: String::from("Snickers"),
        animalType: AnimalType::Dog,
    };

    greet(&dog);
    println!(
        "I can still us dog here: {}, because we just barrowed the dog when passing goh to greet",
        dog.name
    );
```

## Ownership <a name ="ownership"></a>

Rust does not have any garbage collector like languages like `javascript`, `java` ore `golang`. To really understand `rust` we will go through how clearing up memory works in `rust` and how ownership works.
Ownership in `rust` is a strategy for the rust compiler to managing data in memory and preventing common problems.
Every chunk of memory must have one value that is the owner of the memory.
The owner responsibility it to clean up data en free up space(memory).
This happens when the owners variable go out of scope.
We can also decide if the dta should be mutable or not with the `mut` keyword.

Imagine that you inviting your friends for a nice dinner at home. You are responsible for cooking and serving the food, everyone can share and eat the food but you have to clean up and make the dishes when everyone leaves.

Same as for the owner of the variable, we can barrow the data for a bit and then give it back to the owner, for example making a reference to a function parameter.

for example:

```rust
fn say_hi(str: String) {
    println!("hello {} ", str);
}

fn main() {
    let a = String::from("legia");
    say_hi(a);
    // Will give us a error
    println!("{}", a);
}

```

**Why do we borrowing?**
With help of borrowing a value we gain performance. Instead of make a copy of a value and pass it to the function to create a new spot in memory, we can simply borrow the value and then give it back when we are finished.

- What is borrowing? lend out a value instead of transferring ownership
- Why borrow? reduce allocations, improve performance

## Borrowing Patterns <a name = "#borrowing-patterns"> </a>

How does the compiler works when follow different patterns in rust?
Borrowing patterns in rust is great to know to have much better understanding how the complier works, and to learn to work with the complier and not against it.

`At the same time`, what does this actually mean?
Most when you here somone says `Borrow at the same time` they mean they are the same lexical scope created by curly brackets.

```
At the same time = in the same scope
```

```rust
fn main() {
  let xs = vec![1, 2, 3, 4, 5];

  let first = xs.first();
  let last = xs.last();

  println!(
    "The first number is {:?} and the last number is {:?}",
    first, last
  );
}

```

To tell rust that we are done with the immutable borrows before the end of the main, we can add another scope, a inner scope that end before the outer scope.

in Rust > 1.24 this code will compile and works fine.

```rust
  fn main() {
  let mut xs = vec![1, 2, 3, 4, 5];

  let first = xs.first();
  let last = xs.last();

  println!(
    "The first number is {:?} and the last number is {:?}",
    first, last
  );

  *xs.first_mut().expect("list was empty") += 1;
}

```

But in versions < 1.24 we had to wrap our code in local inner scope to make it work.

```rust
fn main() {
  let mut xs = vec![1, 2, 3, 4, 5];

{

  let first = xs.first();
  let last = xs.last();

  println!(
    "The first number is {:?} and the last number is {:?}",
    first, last
  );

 }

  *xs.first_mut().expect("list was empty") += 1;
}

```

the `entry` method in rust defined on a `HashMap`.
This method abstracts away the conditionals that handle if the key is present or not, and instead exposes methods to customize what to do in those cases.

For example this code will not compile

```rust
use std::collections::HashMap;

fn main() {
  let sen = "Legia Waraszawa is the best team in the world!";

  let mut freqs = HashMap::new();
  let one_string: String = sen.split_whitespace().collect();

  for char in one_string.chars() {
    match freqs.get(char) {
      Some(c) => *c += 1,
      None => freqs.insert(char, 1),
    }
  }
}

```

But when using the `entry` method we can easily split up the logic and make the compiler happy again.

```rust
use std::collections::HashMap;

fn main() {
  let sen = "Legia Waraszawa is the best team in the world!";

  let mut freqs = HashMap::new();
  let one_string: String = sen.split_whitespace().collect();

  for char in one_string.chars() {

    *freqs.entry(char).or_insert(0) += 1;
  }
}

```

`entry` works similar to a get method where you provide the key and you will get back both the key and the value pair.

for example

```rust
  freqs.entry('l') // { key: 'l', value: 1 })
```

## Slices <a name ="slices"></a>

Slices is a datatype in `Rust` that has to do with borrowing different values.
**So what is a slice?**
A slice is a data type that always borrow data owned by some other data structure, like for example a `String`, or `u8`.
A slice contains a pointer and a length, The pointer is a reference to the start of the data that the slice contains, and the length is the amount of elements after the start that the slice contains.

How to store a slice in `Rust`.
We create a slice using an ampersand, the variable we're using/referencing, and square brackets containing a range.

```rust
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
    // name_slice is mar and the length is 3
    println!("{}", cell); // cell
    println!("{}", barrow_all_data); // marcell
}

```

We can even use slices from a `vector` ore an `array`. The difference between a vector and a arrray is that an vector can grow in size similar like a array in `JS` and a array in `Rust` has a fixed length.

rust will panic if we try to create a slice which is greater then the length of the data structure itself.

```rust
  let xs = vec![1,2,3];
  let xsSlice = &xs[0..99]; // Panic ! Not alllowed
```

## Self <a href = "self"></a>

When using the `self` keyword `Rust` creates the reference automatically, and it know when we want to make a mutable reference as well.
For example, a user struct where we implement a `birthday` method, this is a good use case when we want to mute the users age.

````rust
  struct User {
    name: String,
    age: u8,
  }

  impl User {
    birthday(&mut self){
        self.age +=1;
    }
  }

  fn main(){

    let mut marcell = User{ name: String::from("Marcell"), age: 24 };
    marcell.birthday();
    // {name: Marcell, age: 25}
  }
This happens because as soon be use `a` as a parameter in `say_hi` function, the `str` argument will take the ownership over a and we will no longer have access to a.

The compiler will help us:

```rust
error[E0382]: borrow of moved value: `a`
 --> src/main.rs:8:20
  |
6 |     let a = String::from("legia");
  |         - move occurs because `a` has type `String`, which does not implement the `Copy` trait
7 |     say_hi(a);
  |            - value moved here
8 |     println!("{}", a);
  |                    ^ value borrowed here after move
````

To make it work we could borrow the string instead by making a reference.

```rust
fn say_hi(str: &String) {
    println!("hello {} ", str);
}

fn main() {
    let a = String::from("legia");
    say_hi(&a);
    println!("{}", a);
}

```

<hr/>

## Result and Options <a name="rao"></a>

How can we use `Result` and `Option` enum in a effective way.

`Result` enum is a enum defined by the standard library.
It has 2 variants. When a function or method returns a value of type `Result`, if everything went well we will get back the `Ok` variant of the result with a value inside.

If something goes wrong, you''ll get back the `Err` variant of the result.

`Options` is similar to `Result` except that instead of giving back a success ore a failure we either have the value or we don't have the value.

Fo example the `vector` has a built in `last` method that returns a `Option`

```rust
  fn main() {
    let xs = vec![1, 2, 3, 4];
    let num_four = xs.last();

    let empty_xs: Vec<u8> = vec![];

    println!("{:?}", num_four); // Some(4)
    println!("{:?}", empty_xs.last()); // None
}

```

`Panics` is for situation that show that we are in a bad state and that the developer has a bug to fix. We stop executing the program and there is now way to recover from the panic so we must go in and fix that.

`Result` are used to situations where the program might take some actions to recover from. Could be for example read from a file, either we have it or not, there are just 2 cases.

There is a method on Result and Option called `expect` that has similar behavior to using match expression when recovering from errors.

```rust
  let string_num: i32 = "10".parse().expect("ooops"); // while compile
  let foo: i32 = "foo".parse().expect("ooops"); // will NOT compile

  println!("{},{}", string_num, foo);
```

<hr/>

## Generics <a name ="generics"></a>

```rust
  struct JiraTask {
    title: String,
    developer: Option<Dev>
  }


```

---

## Functional programming <a name ="functional programming"></a>

Create our own `PURE` map function without mutating the original list.

```rust

fn main() {
    let xs = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result = transform(&xs, double);
    println!("{:?}", result);
    println!("{:?}", xs);
}

fn double(x: &i32) -> i32 {
    x * 2
}

fn transform(xs: &Vec<i32>, func: fn(x: &i32) -> i32) -> Vec<i32> {
    let mut new_list: Vec<i32> = Vec::new();

    for x in xs {
        new_list.push(func(x));
    }

    new_list
}

```

#### [Playground](#playground)

A simple filter even numbers example not using built in filter function.

```rust
fn main() {
    let xs: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

    let evenXs = evens(&xs);
    println!("{:?}", evenXs);
}

fn evens(xs: &Vec<i32>) -> Vec<i32> {
    let mut newXs: Vec<i32> = Vec::new();

    for x in xs.iter() {
        if x % 2 == 0 {
            newXs.push(*x);
        }
    }
    newXs
}
```

#### Higher order functions in `Rust`

```rust
fn main() {
    let cars: Vec<String> = vec!["audi", "mercedes", "bmw", "volvo", "ferrari"]
        .iter()
        .map(|x| x.to_uppercase())
        .collect();

    println!("{:?}", cars);
}
```
