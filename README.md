# New born ruslang

## Table of Contents

- [New born ruslang](#new-born-ruslang)
  - [Table of Contents](#table-of-contents)
  - [About <a name = "about"></a>](#about-)
  - [Statement vs Expression <a name ="s-vs-e"></a>](#statement-vs-expression-)
  - [Control Flow <a name ="cf"></a>](#control-flow-)
  - [Enums <a name ="enums"></a>](#enums-)
  - [Structs <a name ="structs"></a>](#structs-)
  - [Methods <a name ="methods"></a>](#methods-)
  - [Borrowing <a name ="borrowing></a>](#borrowing-a-name-borrowinga)

## About <a name = "about"></a>

New born ruslang, a beginner repo that will go through the very basics in rust such as
`[Control Flow](#cf)`, `[Structs](#structs)` `[Methods](#methods)` and so much more.

Rust is a system level programing language, but it is so much more thane that. Me personally that coming from a `Javascript` background, Rust is a game changer, people think that web-assembly and rust will replace `Javascript` but I don't think it is like that. I know that Rust together with Javascript will make `Javascript` so much more powerful and only the feature can tell us what it will bring.

Javascript developers will use Rust and web assembly without even knowing it, if you don't want of course, for example front end frameworks like `React` or `Svelte` could be even more optimized with `Rust` and thats is what I mean that you will probably use it without even knowing it. The thing you will make notice of is how much faster and precise everything would be.

This Repo is for you as a `JS` developer to learn the fundamentals in `Rust` and how it could help you to become a much better `Javascript` developer

## Statement vs Expression <a name ="s-vs-e"></a>

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

## Control Flow <a name ="cf"></a>

## Enums <a name ="enums"></a>

## Structs <a name ="structs"></a>

## Methods <a name ="methods"></a>

## Borrowing <a name ="borrowing"></a>

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
Like somone want's to borrow your car for a while.
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
}

```

**Why do we borrowing?**
With help of borrowing a value we gain performance. Instead of make a copy of a value and pass it to the function to create a new spot in memory, we can simply borrow the value and then give it back when we are finished.

- What is borrowing? lend out a value instead of transferring ownership
- Why borrow? reduce allocations, improve performance
