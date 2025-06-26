# Trait and Generics

This chapter will backtrack to Rust's type system and explain some of the common hazards and pitfalls 
when working with traits and generics. Though the type system seems flexible, it's still quite static
and needs your attention - that's why Rust is charming.

## Generics

You might've learnt it from C++ or Java. It's pretty similar in Rust.

```rust
fn add<T>(a: T, b: T) -> T {
    a + b // T must implement the `Add` trait
}
```

Here, `T` is a generic type parameter, as the template parameter in C++. Rust can infer the type of `T` most of the time,

```rust
fn main() {
    let a = 1;
    let b = 2;
    let c = add(a, b); // T is inferred as i32
    println!("{} + {} = {}", a, b, c);
}
```

but you can also specify it explicitly:

```rust
add::<i32>(a, b); // T is explicitly specified as i32
```

but not every type implements the `Add` trait, so you need to specify a bound for `T`:

```rust
use std::ops::Add;

fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b // T must implement the `Add` trait
}
```

Generics can also be used with structs and enums, allowing you to define types that can work with any data type.

```rust
struct Point<T> {
    x: T,
    y: T,
}
```

## Trait and `impl`

Traits are a way to define shared behavior in Rust. You can think of them as interfaces in other languages. Here's how you can define a trait:

```rust
trait Shape {
    fn area(&self) -> f64;
}
```

Then, you can implement this trait for different types:

```rust
impl Shape for Point<f64> {
    fn area(&self) -> f64 {
        0.0 // Points don't have an area
    }
}

fn main() {
    let p = Point { x: 1.0, y: 2.0 };
    println!("Area of point: {}", p.area()); // Calls the area method
}
```

As you may notice, `p.area()` looks like a OOP-styled. Rust is not an OOP language but it supports defining functions for a stuct or enum,
which is called `impl` block. You can define methods for a struct or enum in an `impl` block, and you can also implement traits for them.

```rust
impl Point<f64> {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}
```

Besides explicitly implementing traits, Rust also provides a way to implement traits with default methods, using macros.

```rust
#[derive(Debug, Clone, Copy)]
struct Point<T> {
    x: T,
    y: T,
}
```

Now you have `Copy`, `Clone`, and `Debug` traits implemented for `Point<T>`. These dirty works are long gone!

We have to mention another useful trait, `Display` where you can define how a type should be formatted when printed.

```rust
use std::fmt;

impl<T: fmt::Display> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 1.0, y: 2.0 };
    println!("Point: {}", p); // Calls the Display trait
}
```

## Const Generics

Const generics is a feature in Rust that allows you to define a generic type with a constant value.
You can't imagine that before it, Rust actually had a stupid problem

```rust
fn main() {
    let v1 = [1u32; 32];
    let v2 = [1u32; 33];

    println!("{:?}", v1);
    println!("{:?}", v2);
    //               ^^ `[u32; 33]` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
}
```

because Rust's libcore implemented `std::fmt::Debug` for all arrays with size from 0 to 32 but not for 33, by expanding macro.
This seems pretty weird, so Rust introduced a new feature called const generics. And we could imagine that `fmt` becomes simple

```rust
impl<T: fmt::Debug, const N: usize> fmt::Debug for [T; N] {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&&self[..], f)
    }
}
```

As you see, similar to array, const generics can be used to define a generic type with a constant value, like matrix and vector,
which is quite useful in numerical computing. An example from 
https://github.com/getong/rust_example/blob/main/const_workspace_example/const_generics_example/src/main.rs

```rust
use std::{
    fmt::Debug,
    ops::{Add, Mul},
};

#[derive(Copy, Clone, Debug)]
struct Matrix<T: Copy + Debug, const N: usize, const M: usize>([[T; M]; N]);

impl<T: Copy + Debug, const N: usize, const M: usize> Matrix<T, N, M> {
    pub fn new(v: [[T; M]; N]) -> Self {
        Self(v)
    }

    pub fn with_all(v: T) -> Self {
        Self([[v; M]; N])
    }
}

impl<T: Copy + Default + Debug, const N: usize, const M: usize> Default for Matrix<T, N, M> {
    fn default() -> Self {
        Self::with_all(Default::default())
    }
}

impl<T, const N: usize, const M: usize, const L: usize> Mul<Matrix<T, M, L>> for Matrix<T, N, M>
where
    T: Copy + Default + Add<T, Output = T> + Mul<T, Output = T> + Debug,
{
    type Output = Matrix<T, N, L>;

    fn mul(self, rhs: Matrix<T, M, L>) -> Self::Output {
        let mut out: Self::Output = Default::default();

        for r in 0..N {
            for c in 0..M {
                for l in 0..L {
                    out.0[r][l] = out.0[r][l] + self.0[r][c] * rhs.0[c][l];
                }
            }
        }

        out
    }
}

type Vector<T, const N: usize> = Matrix<T, N, 1usize>;

fn main() {
    let m = Matrix::new([[1f64, 0f64, 0f64], [1f64, 2f64, 0f64], [1f64, 2f64, 3f64]]);
    let v = Vector::new([[10f64], [20f64], [40f64]]);

    println!("{:?} * {:?} = {:?}", m, v, m * v);
}
```