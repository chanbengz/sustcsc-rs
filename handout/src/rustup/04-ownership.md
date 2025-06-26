# Ownership and Lifetime

I must admit that ownership and lifetime are very sucky features in Rust, and yet it only does when you
learn it for the first time. Once you get used to it, it is actually quite powerful and useful, in a good
way because it can enforce you to check something that you might forget. Let's jump into it!

## Ownership

We suppose that you know something about stack, heap and memory. You're aware that when writing a program,
you have to think about how to manage memory, like requesting memory, using it, and then releasing it. 
The last step is the most important one, while most of us will neglect it.

> **Bob:** Why would I release memory? We got OS to do that for us, right?
> 
> **Alice:** Nah, Bob. OS recycles your shit only when you stop it.

You have to be programming in a memory-aware way, but it sucks. Your mind might be blown if you're not
experienced with it or you're writing complex code bases. So here's some ways to omit the pain:
- Garbarge Collection (GC): compiler inserts GC code into yours to keep track of the variables while your
program is running.
    - Pros: user friendly
    - Cons: performance loss, incomplete (in some corner cases, it breaks)
- Ownership: compiler checks the usage and ownership of memory and releases it automatically when it's no 
longer needed.
    - Pros: performance (as it's only in compile time), completely memory safe
    - Cons: steep learning curve

Rust takes the ownership model and strictly adhere to it. An example to show how it solves the problem.
Suppose we have a C program that you would probably do

```c
int* foo() {
    int a = 100;
    return &a;
}
```

You allocate a variable `a` and return its pointer, nothing wrong? No! This is a "dangling pointer" because
`a` is allocated on the stack, and when the function returns, `a` is no longer valid. Nobody can use it after
that. So how Rust solves this problem? It doesn't allow you to do that at all. If you try to write the same code in Rust:

```rust
fn foo() -> &i32 {
    let a = 100;
    &a // error: cannot return reference to local variable `a`
}
```

You will get a compile-time error saying that you cannot return a reference to a local variable. When `foo` returns, `a` is
dropped and so the reference to `a` is no longer valid. Rust prevents this by enforcing ownership rules at compile time.
Bare in mind that Rust enforces
1. A memory region (variable/arrays) can only have one owner at a time. Others must borrow it, i.e., request a reference to it.
2. For mutable references, there can only be one mutable reference to a memory region at a time and during the lifetime of that 
   reference, no other references can be made to that memory region.
3. When the owner goes out of scope, the memory region is automatically released (or called dropped in Rust).

When you rewrite it in Rust, you would do

```rust
fn foo() -> i32 {
    let a = 100;
    a
}
```

It works but it looks like C. Did Rust compiler check it? Yes, the blind spot is that `i32` as a primitive type is implicitly copied
when you return it.

## Transfer, Clone and Borrow

Why does this pass the compilation

```rust
let a = 100;
let b = a;
println!("{}", a); // `a` is still valid
```

while this doesn't

```rust
let a = String::from("Hello");
let b = a;
println!("{}", a); // error: value borrowed here after move
```

Did they all transfer ownership? Yes or no. The first one implements a `Copy` trait (will get to that soon), and therefore it is
implicitly copied when you assign it to `b`. Though `a` and `b` are both 100, they are different ones. It's like Java's objects.
Whereas, the `String` type does not implement `Copy`, so when you assign `a` to `b`, the ownership of the memory region is transferred.
When you reuse `a`, it will throw an error because `a` is no longer holding the ownership of the memory region. 

What if `a` and `b` holds the same memory region? Imagine that `a` and `b` are in the same scope and they go out of scope at the same time.
Boom! You have a double free error, and in C/C++, it's like

```c
int* a = malloc(sizeof(int));
int* b = a;
// end of scope
free(a);
free(b); // error: double free
```

It would be critical if the memory of `a` (or `b`) is assigned to another variable, and then you free it, since you're not aware of that.
You could be debugging for hours to find the root cause.

Yes, Copy/Clone can solve this problem. Yet, it's expensive when you copy a large memory region, like a vector or a struct, not to mention
that copying variables will make them independent, so changes to one will not affect the other. If you miss that, you might end up with
unexpected results: why this variable is not updated?

We suggest that you use borrowing for most of the time. Borrowing means that you request a reference to the memory region without taking
ownership of it. It's similar to pointers in C/C++ and references in Java/Python. You can borrow a memory region by using the `&` operator
and after that, dereference it with the `*` operator.

```rust
let a = 114514;
let b = &a; // borrow `a`

assert_eq!(*b, 114514); // dereference `b` to get the value of `a`
assert_eq!(b, 114514);  // don't do this, it will not compile
```

Be aware of the immutablility when you'd change the value of `a`!

```rust
let mut a = 114;

fn change_value(b: &mut i32) {
    *b = 514; // dereference `b` to change the value of `a`
}

fn change_value_wrong(b: &i32) {
    *b = 514; // error: cannot assign to `*b`, as `b` is an immutable reference
}

change_value(&mut a); // borrow `a` mutably
```

Remember that you can only have one mutable reference?

```rust
let mut a = 114;
let b = &mut a; // borrow `a` mutably
let c = &mut a; // error: cannot borrow `a` as mutable more than
let d = &a; // error: cannot borrow `a` as immutable while it is also borrowed as mutable
```

If you know data race, this is exactly what Rust prevents. You cannot have multiple mutable references to the same memory region at the same time,
and you cannot have a mutable reference while there are immutable references to the same memory region. This enforces thread safety. But it's
anoying when you do multi-threading. We will talk about it later.

## Lifetime

> From now on, forget about the `delete` keyword.

As we said, when the owner goes out of scope, the memory region is automatically released. We have an extended example from above

```rust
let a;
{
    let b = String::from("Hello");
    a = &b;
}
println!("{}", a); // error: value borrowed here after move
```

this is because the lifetime of them disagree

```rust
let a;                             // --------+- 'a
{                                  // --+- 'b |
    let b = String::from("Hello"); //   |     |
    a = &b;                        // --+     |
}                                  //         |
a                                  // --------+
```

But ofter, nothing is perfect.

```rust
fn longest_string(s1: &str, s2: &str) -> &str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

let s1 = String::from("Hello World!");
let s2 = String::from("World");
println!("The longest string is: {}", longest_string(&s1, &s2));
```

Oops! Compiler complains that it cannot infer the lifetime of the returned reference. 
Because the lifetime of `s1` and `s2` may be different, and it's runtime dependent. You have to console compiler that you
know what you're doing. You can do that by annotating the lifetime of the references in the function signature.

```rust
fn longest_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

let s1 = String::from("Hello World!");
let s2 = String::from("World");
println!("The longest string is: {}", longest_string(&s1, &s2));
```

By doing this, you tell the compiler that you're pretty sure `s1` will live at least as long as `s2` (or the opposite).

For more advanced lifetime usage, refer to the [Rust Course](https://course.rs/basic/lifetime.html).