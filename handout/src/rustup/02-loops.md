# Loops and Arrays

Loops and arrays are fundamental concepts in all programming languages.
So we will assume that you've already written loops and arrays in other languages.
This section will tell you how to write loops and arrays in Rust and at the last,
we will introduce the `ndarray` crate, which is a powerful library for numerical 
computing in Rust.

## Loop and while

Rust does not define a `for` loop like in C/C++ or Java. Instead, it's like Python's.
For example, to iterate over a range of numbers, you can use the `for` loop like this:

```rust
for i in 0..10 {
    println!("i = {i}");
}
```

which will print numbers from 0 to 9. If you'd include the tail number, you can use `0..=10`:

```rust
for i in 0..=10 {
    println!("i = {i}");
}
```

It's an iterator-like syntax because `..` gives a range iterator just like `range()` in Python.
So similarly, Rust also has
- `x..` to create a slice like `[x:]` in Python, without a right bound.
- `..y` to create a slice like `[:y]` in Python, without a left bound.

Infinite loops can be created using `loop`:

```rust
loop {
    println!("This will run forever!");
    // break;
}
```

## Iterators

The concept of iterators originates from functional programming languages, which will be discussed
in the next chapter. But for now, we just focus on how to use iterators in Rust. To use an iterator,
the stuff you'd iterate over must implement the `Iterator` and `IntoIterator` trait. We don't need to
worry about this here.
- The point of having a `Iterator` is that it implements the `next()` method, which is the core of the
  iterator concept. FYI, an iterator is a stateful object that returns a sequence of values by `next()`.
  You don't need to care about the internal state of the iterator, just call `next()` to get the next value.
- The `IntoIterator` trait is used to convert a collection into an iterator. It provides the `into_iter()` method,
  which returns an iterator over the collection. This is similar to Python's `iter()` function.

For example, the `for` loop we mentioned above can be rewritten using an iterator:

```rust
let mut iter = (0..10).into_iter();
while let Some(i) = iter.next() { // Some<T> is exlained in the next chapter
    println!("i = {i}");
}
```

and using iterators can give you more flexibility, such as filtering

```rust
let mut iter = (0..10).into_iter().filter(|&x| x % 2 == 0)
while let Some(i) = iter.next() {
    println!("i = {i}"); // i is even
}
```

or stepping through the range:

```rust
for i in (0..10).into_iter().step_by(2) {
    println!("i = {i}"); // i is even
}
```

Why do we need iterators? Because they are lazy! They don't compute the values until you actually need them.
Second, they can be easily composed together to create complex data processing pipelines. We will see a more
advanced example in the next chapter when we talk about functional programming.

## Array (Slice and Vec)

Array in Rust is a fixed-size collection of elements of the same type. It is similar to arrays in C/C++ or Java
but it comes with a formal definition: `[T; N]`, where `T` is the type of the elements and `N` is the size of the array 
(it's a generic). So array with different sizes are also different types. Use array as you want

```rust
let mut arr: [i32; 5] = [0; 5]; // an array of 5 i32 initialized to 0
//  ^^^ mut is important
let arr2 = [1, 2, 3, 4, 5];

arr[0] = 1; // set the first element to 1
println!("{:?}", arr); // print the array
```

To iterate over an array, you can use a `for` loop:

```rust
for i in &arr { // note the reference
    println!("i = {i}");
}
```

Slicing is a way to create a view into a part of an array. It has the annotation `&[T]`, which is a reference to a slice 
of type `T` and looks like Python's.

```rust
let a = [1, 2, 3, 4, 5];
let s = &a[1..4]; // slice from index 1 to 3 (4 is excluded)
assert_eq!(s, [2, 3, 4]);
```

You may notice the reference `&` in the slice. This is because Rust is strict about ownership and borrowing, which will
be discussed in the next chapter. For now, just remember that slices are references to parts of arrays.

To create a dynamic array, Rust provides the `Vec` type, which is a growable array. Here's how you can use it:

```rust
let mut v: Vec<i32> = Vec::new(); // create an empty vector
v.push(1); // add an element to the vector
v.push(2);
v.push(3); // add more elements
println!("{:?}", v); // print the vector

let mut v = vec![1, 2, 3]; // create a vector with initial elements
v.push(4); // add an element to the vector
println!("{:?}", v); // print the vector

let mut v = vec![0; 5]; // create a vector of 5 elements initialized to 0
v[0] = 1; // set the first element to 1
println!("{:?}", v); // print the vector
```

And to play vector with iterator, you can

```rust
let v: Vec<_> = (114..514).into_iter().collect();
// or a generic way
let v = (114..514).into_iter().collect::<Vec<_>>();
```

As in other vectors (C++/Java/Python), the dynamic array is costly to resize because of `remalloc` and moving data,
so it will be better to preallocate the size of the vector if you know it in advance:

```rust
let mut v: Vec<i32> = Vec::with_capacity(100); // create a vector with capacity 100
for i in 0..100 {
    v.push(i); // add elements to the vector
}
```

## ndarray

It's good to have the native `Array` and `Vec` types in Rust, but they are not enough for numerical computing.
For example, creating a 2D array (matrix) is not straightforward and not efficient with the native types

```rust
let mut mat = vec![vec![0; 3]; 4]; // a 4x3 matrix initialized to 0
mat[0][0] = 1; // set the first element to 1
println!("{:?}", mat); // print the matrix
```

Too much vector nesting, and the performance is not good because of the memory layout. A intuitive way to create
a 2D array is flatten it into a 1D array and wrap it in a struct with metadata

```rust
struct Matrix {
    data: Vec<i32>, // the data of the matrix
    rows: usize,   // the number of rows
    cols: usize,   // the number of columns
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            data: vec![0; rows * cols], // initialize the data to 0
            rows,
            cols,
        }
    }
    // yeah I hate the getter and setter
    // we can write in a better way but it's not the point here
    fn get(&self, row: usize, col: usize) -> i32 {
        self.data[row * self.cols + col] // access the element at (row, col)
    }

    fn set(&mut self, row: usize, col: usize, value: i32) {
        self.data[row * self.cols + col] = value; // set the element at (row, col)
    }
}
```

and that's what `ndarray` crate does for you. To equip with it, execute in the terminal

```bash
cargo add ndarray
```

then, you can use it like this:

```rust
use ndarray::{array, Array2};

let a = Array2::zeros((4, 3)); // create a 4x3 matrix filled with zeros
println!("{:?}", a);

let b = array![[1, 2, 3], [4, 5, 6]]; // create a 2D array (matrix) with initial values
println!("{:?}", b);
```

index the matrix is also straightforward:

```rust
let a = array![[1, 2, 3], [4, 5, 6]];
println!("a[0, 0] = {}", a[[0, 0]]);
```

For slicing and iterating, `ndarray` provides a lot of methods to manipulate the array easily:

```rust
use ndarray::{array, s};

let a = array![[1, 2, 3], [4, 5, 6]];
let b = a.slice(s![.., 0..1, ..]);
```

more about slicing and iterating can be found in the `ndarray` documentation.

Moreover, `ndarray` provides lots of optimizations! View it next chapter.

## String

String is complicated in Rust so we won't talk about it in detail here.
Refer to [this](https://zhuanlan.zhihu.com/p/658120118).