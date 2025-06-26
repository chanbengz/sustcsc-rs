# Tuning Rust Code for Performance

Since most of you guys know little about OS, we won't talk much about low-level threading concepts, like
synchronization, memory barriers and so on. We will only focus on embarrassingly parallel problems, leveraging
Rust's thread safety. Next, we will have a simple look at SIMD, and yet this does not require you to handwrite
SIMD code, as Rust provides a good abstraction for it. Finally, we will talk about some ad-hoc optimizations
for this challenge.

## Parallelism

Everything we talked so far is single-threaded, which is not enough for modern computing. We need to leverage
multiple cores to speed up our code. Rust provides a simple way to do this, using the `std::thread` module. But
this is not intuitive at first, so will use another one called `rayon`. This one provides easy-to-use parallel iterators.

Before we start, you need to add `rayon` to your `Cargo.toml`:

```bash
cargo add rayon
```

Then you can use it like this:

```rust
use rayon::prelude::*;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let doubled: Vec<_> = data.par_iter().map(|&x| x * 2).collect();
}
```

This snippet will run `map` in parallel, as the elements in `data` are independent of each other. This is called 
embarrassingly parallel, which means that the problem can be easily divided into smaller independent tasks. If you
need to create independent tasks other than the iterators, you can use `rayon::spawn`:

```rust
use rayon::prelude::*;

fn main() {
    let data = vec![5, 1, 8, 22, 0, 44];
    rayon::join(
        || println!("Sum: {}", foo(&data)),
        || println!("Double Sum: {}", bar(&data)),
    );
}

fn foo(arr: &Vec<i32>) -> i32 {
    arr.iter().sum()
}

fn bar(arr: &Vec<i32>) -> i32 {
    arr.map(|x| x * 2).iter().sum()
}
```

To further optimize rayon, we have to understand where the bottleneck is. One problem is that the workload above
is trivial and rayon just simply splits them into separate threads. If `context switching` pops out in your mind,
you are right. The overhead is largely due to the uncessary threads and their costs. To avoid this, we can use
`par_chunks` to split the data into chunks in one thread each to keep cores busy

```rust
use rayon::prelude::*;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let doubled: Vec<_> = data.par_chunks(1024).flat_map(|chunk| {
        chunk.iter().map(|&x| x * 2)
    }).collect();
}
```

## SIMD

Single Instruction Multiple Data (SIMD) is a low-level technique in CPU. Since most of you are not familiar with
computer architecture, we won't go that deep. To explain it in a few sentences, SIMD unrolls the loop and executes
the same operation on multiple data at once. It's pretty like rayon, but at a lower level. This is a powerful 
technique to speed up numerical computing, but unfortunately, Rust has not fully supported it yet. To turn on SIMD, 
you need to switch to the nightly version of Rust and enable features flags.

```bash
rustup default nightly
```

The simplest way to use SIMD in Rust is to hint compiler to use SIMD instructions but for now, compiler isn't smart enough to
do this automatically. Also, SIMD has its limitations, like it works at maximum speed only when the data is aligned and independent.

The current simplest way to use SIMD in Rust is `portable-simd`. Let's take `sum` again as the example

```rust
#![feature(portable_simd)]  // Release the nightly compiler kraken
use multiversion::{multiversion, target::selected_target};
use std::simd::prelude::*;

#[multiversion(targets("x86_64+avx2+fma", "x86_64+avx", "x86_64+sse2"))]
fn simd_sum(x: &Vec<f32>) -> f32 {
    const SIMD_WIDTH: usize = const { 
        selected_target!().suggested_simd_width::<f32>().unwrap_or(1)
    };

    let (peel, body, tail) = x.as_simd::<SIMD_WIDTH>();
    let simd_sum = body.into_iter().sum::<Simd<f32, SIMD_WIDTH>>();
    let scalar_sum = peel.into_iter().chain(tail).sum::<f32>();
    simd_sum.reduce_sum() + scalar_sum
}
```

The worst way but the most compatible way to use SIMD in Rust is to use `std::arch` module. And you need to handwrite
assembly code. Well, you don't need to do this in this challenge, but it's good to know that you can do it if you want.

## Compilation

The last thing we need to talk about is the compilation. Rust has a powerful optimization system, and you can
enable it by setting the `release` profile in your `Cargo.toml`:

```toml
[profile.release]
opt-level = 3
lto = "fat"
```

`opt-level` controls the optimization level made automatically by the comiler, and `lto` enables link-time optimization, which
allows the compiler to optimize across crate boundaries, i.e., by thinking about the program as a whole. Meanwhile, you can 
enable feature flags to optimize the code further. For example, you can enable `simd` feature flag to use SIMD instructions
in your code. You can also use `multiversion` to enable multiple versions of the same function for different targets, which can
be useful for performance optimization.

When compiling, use the `--release` flag to enable the release profile:

```bash
cargo build --release
```