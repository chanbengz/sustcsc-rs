# Hints

Something to help you get started with the exercises in this section.
First of all, you should already know what this challenge requires you to do,
and download the starter code from the repos.

## Parallelization

Everything in the starter code except the `main.rs` is parallelizable and modifiable.
For example, the encryption and decryption simply iterate the grid and encrypt/decrypt
each cell, so you can use `par_iter` to parallelize the process. The same goes for the
`update_grid` function, which updates the grid based on the rules of the Game of Life.

## Simple SIMD

You don't need to handwrite the SIMD code, since the `tfhe-rs` library already provides
a SIMD implementation for the LWE encryption scheme. Search for how to enable this.

## Compilation

You know what to do. 

## Ad-hoc optimizations

One of the most effective optimizations I can think of is to clone the `tfhe-rs` and
optimize it, but it's not really necessary for this challenge. The second best way would
be to benchmark why the starter code is slow, like the computation cost for the FHE types
like `FheUInt8`.

Meanwhile, there're something unnecessary or written in a dumb way, such as unnecessary cloning.
You should remove it.

## (Bonus) Switching to different algorithms

The Babai's nearest plane algorithm is not the best algorithm for the LWE decryption.
I've noticed that we have other algorithms that might work better, including but not limited to:
- BKZ algorithm
- pnj-bkz
- Sieve (G6k)
- You tell me.

I'm pretty excited to see what you can come up with. Surprise me!