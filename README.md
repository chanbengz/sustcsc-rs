# SUSTCSC 2025 - Rust for HPC

> [!NOTE]
> Designer: Ben Chen \<chenb2022@mail.sustech.edu.cn\>
> 
> Platform: Xeon Platinum CPU

## Structure

In this repos you'll find:
- [Starter's Code](./challenge): LLL algorithm attack on LWE
- [Handout](./handout): CI/CD will automatically build sites with `mdbook`, so you find it [here](https://sustcsc25.benx.dev).

## Description

Learning With Errors (LWE) is a hard problem in lattice-based cryptography invented for the post-quantum era.
The LWE problem is to find a secret vector $\mathbf{s} \in \mathbb{Z}_q^n$ given a set of noisy linear equations of the form
$$
\mathbf{A} \cdot \mathbf{s} + e = b \mod q
$$
where $\mathbf{A} \in \mathbb{Z}_q^{m \times n}$ is a matrix, $e$ is a noise vector,
 and $b$ is the result vector. We call $\mathbf{A}$ the lattice basis, $e$ the noise, and $b$ the ciphertext. The LWE problem is hard to solve when the noise is small compared to the modulus $q$.

In this challenge, 

To evaluate the performance of your code, we will use the following metrics:
- You have to solve each given LWE problem with a upper bound of computation time $T$.
- If you finish all the tasks in the given time, the faster the better.

## Environment

See https://sustcsc25.benx.dev/setup/00-overview.html for setup instructions.

TBD

If having trouble with the setup or machine, please contact us.

## Benchmarking

> [!WARNING]
> FFI and binding are banned, while one can insert inline assembly. Keep your unsafe code minimal.
> 
> We will benchmark your code in a docker container with native CPU but no GPU support. So don't try
> to use Rust-CUDA or any other GPU libraries. Container has not internet access, so don't try to accelerate
> your code by remote machines, whereas you can use crates. 
> 
> Build scripts are not allowed as it's counted in compilation time.

### Test Cases

TBD

### Compilation

TBD

### Accuracy

If the result has $\epsilon \geq \alpha$, we will consider it as a failure.

## Submission

You have to submit your code along with a report by
- Compressed file, either `tar`, `gzip`, `zip`, or `7z`.
- A Link to your public repository, if you forked this repository.

Submitted files may look like
```
/sustcsc25-<your teamid>
├── challenge
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       ├── lwe.rs    # Encrypt/Decrypt Oracle
│       ├── solver.rs # Your solver
│       └── main.rs   # We ignore your main.rs since its judge
├── README.md # Tell us about how to run your code
└── report.pdf
```

### Report

Your report, in English or Chinese, should be a PDF file compiled by $\LaTeX$, markdown,
Typst or any other format that generates PDF.

And it may include:
- Your optimization strategy, e.g., algorithm improvements, hardware features, etc.
- Performance analysis, e.g., profiling, flamegraph, etc.

## Reference

[Numerical Computing with Rust on CPU](https://plmlab.math.cnrs.fr/grasland/numerical-rust-cpu)
[LWE Challenge by TU Darmstadt](https://www.latticechallenge.org/lwe_challenge/challenge.php)