# [SUSTCSC '25] Rust in HPC

In this challenge, you will get hands-on experience using Rust for high-performance computing (HPC) tasks, especially in
a narrow domain of numerical computing. As I only have knowledge about security and cryptography, this challenge will only
focus on acclerating an encrypted numerical computation, or boosting encryption itself task using Rust - most ubiquitous
libraries of cryptography are written in Rust.

## TL;DR

The basic challenge is to boost a encrypted numerical computation task: Conway's Game of Life (2D version) computed on a
fully homomorphic encryption (FHE) scheme (empowered by [tfhe-rs](https://github.com/zama-ai/tfhe-rs)). The scheme is 
implemented with LWE (Learning With Errors, in case you don't know what it is, it's a post-quantum cryptography that is
very complicated in computation, so very slow). Though the task seems tricky (but you don't need to know about the encryption),
you just need to focus on the parallelization, compilation and SIMD optimization of the code - simple as that. FHE is
being developed to be used in real-world secure HPC applications, and demanded by the enterprises.

Once you done the basic one, I've prepared a bonus challenge for you: boost the attack on the LWE scheme, by recovering
the secret vector. This is a very hard one as I don't know the solution neither. But the point is that you explore an
unknown area of the problem space and potentially discover new techniques or optimizations, which can be shown by your
report. I'm expecting you to implement an attack purely in Rust, which I'm sure that no one has done before.

## What's in this website?

You will find the following contents in this website:
- [Setting up Environment](./setup/00-overview.html) will guide you through the preparation for the challenge.
- [Crash Course on Rust](./rustup/00-first-look.html) will get you started with Rust and how it's used in HPC.
- [Introduction to LWE](./lwe/00-introduction.html) will introduce the details of LWE and how it works.

You know what Conway's Game of Life is, so I won't bother to explain it here.