# Functions and Functional Programming

This section may take longer than the others, as we have lots of examples to go through.
It also contains plenty of concepts regarding functional programming, which you may not be familiar with.
And this section isn't just about functions, despite its title, it also covers functional programming in Rust.
So take your time to read through it, and don't hesitate to ask questions if you have any.

## Functions, `Option` and `Result`

Let's assume you know what functions are, and how to define and use them. In Java, it has alias `method`.
Rust defines functions like this:

```rust
fn function_name(parameter1: Type1, parameter2: Type2) -> ReturnType {
    // function body
    // ...
    return value; // or just `value` if the return type is not `()`
}
```

If you don't want to return a value, you can omit the return type, or use `()`, which is the unit type in Rust,
similar to `void` in C/C++ or Java.

Yet, where Rust is special is that its grammar allows you to omit the `return` keyword when returning a value,
as long as the last expression does not end with a semicolon.

```rust
fn sum(a: i32, b: i32) -> i32 {
    // ...
    a + b // no semicolon, so this is the return value
}
```

This kind of syntax can also be seen in `if` statements and `match` expressions, which we will see later.

There're special types in Rust that define how Rust code looks like, the most classic ones are `Option` and `Result`.

If you know functional programming, you may have heard of `Maybe` and `Either` types, and `Option` is one of them - it's a
[monad](https://en.wikipedia.org/wiki/Monad_(category_theory)). But I guess you're less likely to have heard of `monad` so
you can just forget about `monad`. Simply speaking, `Option` is a type that can either be `Some(value)` or `None`.

```rust
fn get_value() -> Option<i32> {
    // ...
    if some_condition {
        Some(42) // return a value wrapped in `Some`
    } else {
        None // return `None` if the condition is not met
    }
}
```

It's often a placeholder for a value that may not exist, or a value that may fail to be computed, but since Rust is 
statically typed, you must return the same type every time, so you cannot return `i32` and `None` at the same time [^1].
Also, to use the value inside an `Option`, you must unwrap it

```rust
let a = Option::Some(114);
let b = Option::Some(514);
let c = a + b; // this will not compile, as `Option` does not implement `Add` trait
let c = a.unwrap() + b.unwrap(); // this will compile, but it may panic if `a` or `b` is `None`
```

[^1]: Otherwise you would have to expicitly define a error value like `-1` or `0`, but sometimes they have special meanings other
than "error", so it's not a good idea to use them as error values.

As for `Result`, it's a type that can either be `Ok(value)` or `Err(error)`, and it's used to represent the result of
an execution that may fail. It's useful if you don't like the panic mechanism in Rust because it aborts the program. Yes, it's
Rust way to deal with `try-catch` in other languages, but it does not have `try-catch` at all.

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string()) // return an error if b is zero
    } else {
        Ok(a / b) // return the result wrapped in `Ok`
    }
}

let result = divide(10, 2);
println!("{:?}", result); // prints `Ok(5)`
let result = divide(10, 0);
println!("{:?}", result); // prints `Err("Cannot divide by zero")`
```

to handle `Err` or `None`, we suggest you use `unwrap_or` or `unwrap_or_else` methods, which will return a default value if the
value is `None` or `Err`.

```rust
let a = Option::None;
let b = a.unwrap_or(0); // if `a` is `None`, return `0` instead of panicking
```

A special case when using `Result` is that if you have nested function calls that share the same error type, for example:

```rust
fn foo() -> Result<i32, String> {
    let a = bar()?; // `bar` returns `Result<i32, String>`
    //           ^ this `?` operator will automatically propagate the error if `bar` returns `Err`
}

fn bar() -> Result<i32, String> {
    // ...
    Err("An error occurred".to_string())
}
```

`?` can be abbreviated as `unwrap_or_else(|e| Err(e))`, which will return the error if it exists and abort the function execution.

## `if let` and `match`

Before `match`, we should bring up `enum` because it's a powerful feature in Rust that allows you to define a wrapped type.

```rust
enum Number {
    Integer(i32),
    Float(f64),
    Complex(f64, f64),
    Rational(i32, i32),
    Zero,
    Nothing
}

fn print_number(num: Number) {
    match num {
        Number::Integer(i) => println!("Integer: {}", i),
        Number::Float(f) => println!("Float: {}", f),
        Number::Complex(r, i) => println!("Complex: {} + {}i", r, i),
        Number::Rational(n, d) => println!("Rational: {}/{}", n, d),
        Number::Zero => println!("0"),
        Number::Nothing => println!("Nothing"),
    }
}
```

sometimes you're tired of exhausting all the variants of an `enum`, Rust provides a convenient way to match `_` as a don't-care pattern,

```rust
fn foo(num: Number) -> i32 {
    match num {
        Number::Integer(i) => i * 2,
        Number::Float(f) => f as i32 * 2,
        Number::Complex(r, i) => (r + i) as i32 * 2,
        _ => 0, // catch-all pattern
    }
}
```

`match` can almost match anything, and more flexible than you can imagine, but we don't elaborate on it here. Refer to
[Rust 圣经](https://course.rs/basic/match-pattern/all-patterns.html)

When you only have one specific variant to match, you can use `if let` to simplify the code:

```rust
fn print_number(num: Number) {
    if let Number::Integer(i) = num { // if let Variant::Value(v) = expression
        println!("Integer: {}", i);
    }
}

print_number(Number::Integer(42)); // prints "Integer: 42"
print_number(Number::Float(3.14)); // does nothing, as the pattern does not match
```

this snippet only matches `Number::Integer`, so it will not print anything else. Similarly, we have `while-let` to match patterns in a loop,
which you've seen in the previous section.

```rust
let mut numbers = vec![Number::Integer(1), Number::Float(2.0), Number::Complex(3.0, 4.0)];
while let Some(num) = numbers.pop() {
    if let Number::Integer(i) = num {
        println!("Integer: {}", i);
    }
}
```

## Iterators

Rust's developers are experienced in OCaml (a functional programming language, opposed to imperative languages like C or Java), 
and they have brought some of the functional programming features to Rust. One of them is iterators. This section elaborates on
aforementioned `Iterator` trait, beyond `next`, `filter` and `collect` methods. If you know functional programming, you may have heard of
`map`, `sum`, `fold`, `zip` and other higher-order functions, which are also available in Rust's iterators.

```rust
let numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
println!("{:?}", doubled); // prints [2, 4, 6, 8, 10]
```

`map` takes a lambda function (or called closure in FP, we'll get to that soon) as an argument and applies it to each element of the iterator,
returning a new iterator. The `collect` method is used to convert the iterator back to a collection, in this case, a `Vec<i32>`. Also, we
have `sum` which is common when computing a vector's norm

```rust
let arr = vec![1, 2, 3, 4, 5];
let norm: f64 = arr.iter().map(|x| x * x).sum().sqrt();
println!("Norm: {}", norm);
```

You can also use `fold` or `reduce` to accumulate values to extend `sum`'s functionality, which is a more general form of `sum`.

```rust
let sum: i32 = arr.iter().fold(0, |acc, x| acc + x); // 0 is the initial value, `acc` is the accumulator
println!("Sum: {}", sum);
let sum: i32 = arr.iter().reduce(|acc, x| acc + x); // `reduce` is similar to `fold`, but it does not take an initial value
```

When you need to iterate over two or more iterators at the same time, you can use `zip` to combine them into a single iterator.
For example, computing a dot product of two vectors:

```rust
let vec1 = vec![1, 2, 3];
let vec2 = vec![4, 5, 6];
let dot_product: i32 = vec1.iter().zip(vec2.iter()).map(|(x, y)| x * y).sum();
println!("Dot product: {}", dot_product);
```

## Lambda and Closure 

Rust is neither a purely functional programming language nor an imperative one. It supports both paradigms, and as a FP language, 
it has first-class functions. A typical example is the lambda, a.k.a. anonymous function, which does not have a name and can capture
variables from its surrounding scope (note that Rust is hard to define global variables, so closures are extremely useful).

```rust
let add = |x: i32, y: i32| x + y; // define a lambda that adds two numbers
let result = add(2, 3);
println!("Result: {}", result); // prints "Result: 5"
```

As previously mentioned, like in `map`, `||` is embraces the parameters of the lambda, and the body is defined after the `|`s.
You can define a more complicated lambda body

```rust
let multiply = |x: i32, y: i32| {
    let result = x * y;
    result // return the result
};
let result = multiply(2, 3);
println!("Result: {}", result);
```

When you try to capture variables from the surrounding scope, it's called a closure, for example:

```rust
let factor = 2;
let multiply = |x: i32| x * factor; // capture `factor` from
println!("Result: {}", multiply(3)); // prints "Result: 6"
```

However, sometimes you may want to take the ownership of the captured variables, for example, you access something that cannot be referenced

```rust
let s = String::from("Hello");
let closure = || println!("{}", s); // this will not compile, as println will take ownership of `s`
```

`move` keyword is here to help you, it will take the ownership of the captured variables (i.e. the variables that are used inside the closure).

```rust
let s = String::from("Hello");
let closure = move || println!("{}", s); // now `s` is moved into the closure
closure(); // prints "Hello"
// println!("{}", s); 
```

## Higher-Order Functions

In Rust, functions have types and traits, and you can pass functions as arguments to other functions, or return functions from functions.
Traits of functions are defined as `Fn`, `FnMut`, and `FnOnce`, which are similar to the function types in other languages. They're advanced
topics so we won't cover them here, but you can read the [Rust documentation](https://doc.rust-lang.org/std/ops/trait.Fn.html) for more information.

Types of functions are defined by the types of their parameters and return values only. You can define different names for the same function type,
and they're the same function type as long as their signatures match. Here's a simple example of a higher-order function that takes a function as an argument:

```rust
fn apply<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32, // F is a function that takes an i32 and returns an i32
{
    f(x)
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn multiply_by_two(x: i32) -> i32 {
    x * 2
}

fn main() {
    let result1 = apply(add_one, 5); // applies `add_one` to 5
    let result2 = apply(multiply_by_two, 5); // applies `multiply_by_two` to 5
    let result3 = apply(|x| x + 3, 5); // applies an anonymous function to 5
    println!("Result1: {}, Result2: {}, Result3: {}", result1, result2, result3); // prints "Result1: 6, Result2: 10, Result3: 8"
}
```