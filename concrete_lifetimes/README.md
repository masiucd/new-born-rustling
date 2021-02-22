# Concrete lifetimes

What is a lifetime in `Rust`?

In `Rust` a values lifetime is the time during which the value exists at a given time on a particular memory address.
You can think about it similar like your home address.
When you were born you and your parents lived a specific address. And a few years later you moved out.
If someone had your old address adn tried to visit you would not be there, because you would be on a different address.

value in code starts when a value is created or moved into a particular location in memory, adn ends when the value is moved of or dropped from that location.

<img src="./rust-life.svg" />
<img src="./carbon.svg" />
