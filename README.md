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
  - [Borrowing <a name ="borrowing"></a>](#borrowing-)
  - [Slices <a name ="slices"></a>](#slices-)
  - [Self <a href = "self"></a>](#self-)
  - [Ownership <a name ="ownership"></a>](#ownership-)

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

`rust`
`rust`

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
Like someone want's to borrow your car for a while.
Even if your friend is using your car it is still your car and your responsibility to pay the taxes and stuff, in this case when it come to `rust` clean up after you.

````rust
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

````

**Why do we borrowing?**
With help of borrowing a value we gain performance. Instead of make a copy of a value and pass it to the function to create a new spot in memory, we can simply borrow the value and then give it back when we are finished.

- What is borrowing? lend out a value instead of transferring ownership
- Why borrow? reduce allocations, improve performance

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
