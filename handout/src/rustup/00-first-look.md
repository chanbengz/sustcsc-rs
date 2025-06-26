# First Look at Rust

Welcome to the world of Rust. The following chapters will lead you to write code in Rust for computing.
Nevertheless, I shall note that Rust is a little bie more complicated than other languages.

> In this page, you can run the code snippets by clicking the "Run" button on the top right corner of the code block.

## Hello World

If you follow the installation guidelines, you should see `cargo` and `rustc` on your machine. Now,
create your first Rust project by

```bash
$ cargo new hello-world --bin
```

Here, `--bin` means that you want to create a standalone project (i.e. runs on its own), so if you'd like to
create a library, you can use `--lib` instead. The project will be created in the `hello-world` directory.

```bash
$ cd hello_world
$ tree .
.
├── Cargo.toml
└── src
    └── main.rs
```

`Cargo.toml` is the configuration file for your project, and you don't need to edit it yourself. The `src` directory
contains the source code of your project. The `main.rs` file is the entry point of your program, and it contains
the `main` function, which is the first function that is called when your program runs, just like in C/C++.

```rust
fn main() {
    println!("Hello world!");
}
```


Here're somethign to take away
- `fn` is used to define a function. We will talk about functions in [Functions](./03-functions.html) section.
- `println!` is a macro that prints the string to the standard output. The `!` indicates that it is a macro, not a function.
   The reason for `print` being a macro is complicated, but you can find clues in this website.
- unlike C/C++, you don't need to `return 0;` at the end of the `main` function. Rust has a better way.

## Variables

Rust has a type inference for declaring variables, so you don't need to specify the type of the variable when you declare it. For example,

```rust
fn main() {
    let x = 5;
    let s = "Hello world!";
    let v = vec![1, 2, 3]; // dynamic array

    println!("x = {}", x);
    println!("s = {}", s);
    println!("v = {:?}", v);
}
```

## Hinted about the type

Rust is a statically typed language and it offers a strong type inference system.
You would probably notice that `let` doesn't require you to specify the type of the variable
as in `Java` or `C`. But type inference is not always possible and so you should hint
the compiler about the type of the variable.

```rust
let x: i32 = 5; // explicitly hint the type of x
let y = 10u32;  // hint by constants
let arr = vec![0.0; 4] // we'll get to vector later
```

## Oops! Inference breaks

There're cases where the compiler cannot know the type of a variable at compile time.
For example, if you declare a variable without initializing it, the compiler will not be able to infer its type:

```rust
let x;
let v = Vec::new(); // or
let v = vec![];
```

You can see the error by clicking the "Run" button above.

To address this, you need to specify the type of the variable explicitly:

```rust
let x: i32;
let v: Vec<i32> = Vec::new(); // or
let v: Vec<i32> = vec![]; // or
let v = Vec::<i32>::new(); // generic
```

## First glance on ownership

Rust has a unique ownership system that enforces you to manage memory safely and efficiently.
When we say define a variable, we actually mean to "bind" a name to a value in Rust.
For example, if you try to use a variable after it has been moved, the compiler will throw an error:

```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // move ownership from s1 to s2
    println!("{}", s1); // this will not compile, as s1 is no longer valid
}
```

because `s1` is no longer valid after the ownership has been moved to `s2`.
To address this, you can use either cloning or borrowing (reference) to avoid take ownership of the variable.

```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone(); // clone the value of s1 to s2
    let s3 = &s1; // borrow a reference to s1
    println!("{}", s1); // this will compile, as s1 is still valid
}
```

## Printing

Rust formatting is similar to other languages. If you have some experience with other languages, you
would probably know that not all the types can be printed directly. For example, if you try to print
a self-defined struct, you will get a weird output in Python:

```python
class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

if __name__ == "__main__":
    p = Point(1, 2)
    print(p)
```
This will print something like `<__main__.Point object at 0x7f8c2c3e4d90>`, which is not very useful. 
Python has a special method `__str__` and `__repr__` to address this, and Java has `toString()`, etc.
In Rust, you can use the `Debug` trait to print the struct. You need to derive the `Debug` trait for
your struct, and then you can use the `{:?}` format specifier to print it. For example:

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{:?}", p);
}
```

Or if you want to print it in a more human-readable format, you can use the `{}` format specifier and
implement the `Display` trait for your struct. For example:

```rust
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y) // just like a write syscall
    }
}
```

Then you can use the `{}` format specifier to print it:

```rust
fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{}", p); // (1, 2)
    // or
    let p = Point { x: 1, y: 2 };
    println!("{p}"); // (1, 2)
}
```

and a more complex formatting:

```rust
fn main() {
    println!("x = {x}, y = {y}, sum = {sum}", x=1, y=2, sum=1+2);
}
```