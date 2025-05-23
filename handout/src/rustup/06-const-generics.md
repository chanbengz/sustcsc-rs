# Trait and Generics

## Trait

## Generics

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