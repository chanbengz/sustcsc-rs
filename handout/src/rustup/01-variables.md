# Variables and Numbers

This chapter tells about how you can compute with numbers in Rust.

## Mutable and Immutable Variables

In Rust, variables are immutable by default. This means that once a variable is bound to a value, it cannot be changed. For example:

```rust
fn main() {
    let x = 5;
    println!("x = {}", x);
    x = 6; // error: cannot assign twice to immutable variable `x`
}
```

If you want to make a variable mutable, you can use the `mut` keyword:

```rust
fn main() {
    let mut x = 5;
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);
}
```

In this case, `x` is mutable, so you can change its value.

## Constants

Constants are similar to immutable variables, but they must be explicitly typed and cannot be changed.
You can define a constant using the `const` keyword:

```rust
const PI: f64 = 3.141592653589793;
```

Constants are always immutable and must have a type annotation. They can be defined in any scope, including inside functions.

```rust
fn main() {
    const PI: f64 = 3.141592653589793;
    println!("PI = {}", PI);
}
```

## Numbers

Rust has a intuitive name for the number types, like `i32` for 32-bit signed integers, `u32` for 32-bit unsigned integers, and `f64` for 64-bit floating-point numbers. You can use the following types:

```rust
fn main() {
    let x = 5i32;
    let y = 10u32;
    let z = 3.14f64;

    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
}
```

## Struct

You can also define your own types using `struct`. A struct is a custom data type that lets you package together related data. Here's an example of a simple struct:

```rust
struct Point {
    x: f64,
    y: f64,
    pub metadata: String, 
    // `pub` makes this field accessible outside the module
}
```

to create an instance of a struct, you can use the following syntax:

```rust
fn main() {
    let p = Point {
        x: 1.0,
        y: 2.0,
        metadata: String::from("A point in 2D space"),
    };
    println!("Point: ({}, {}), Metadata: {}", p.x, p.y, p.metadata);
    //                                        ^^^ not accessible outside the module
}
```

## Tuple

Tuple is a fixed-size collection of values of different types. You can create a tuple using parentheses:

```rust
fn main() {
    let tuple: (i32, f64, char) = (42, 3.14, 'a');
    println!("Tuple: ({}, {}, {})", tuple.0, tuple.1, tuple.2);
}
```

Tuples are the same typed if all their elements have the same type.

## Enum

Enums are a way to define a type that can be one of several different variants. You can define an enum using the `enum` keyword:

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
```

You can wrap data in an enum variant:

```rust
enum Card {
    Clubs(u8), // Clubs with a value
    Diamonds(u8), // Diamonds with a value
    Hearts(u8), // Hearts with a value
    Spades(u8), // Spades with a value
}

let card = Card::Hearts(10);
match card {
    Card::Clubs(value) => println!("Clubs with value: {}", value),
    Card::Diamonds(value) => println!("Diamonds with value: {}", value),
    Card::Hearts(value) => println!("Hearts with value: {}", value),
    Card::Spades(value) => println!("Spades with value: {}", value),
}
```