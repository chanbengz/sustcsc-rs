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
