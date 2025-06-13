# SUSTCSC 2025 - Rust for HPC

> [!NOTE]
> Contact: Ben Chen \<chenb2022@mail.sustech.edu.cn\>
> 
> Platform: Xeon Platinum CPU

## Structure

In this repos you'll find:
- [Starter's Code](./challenge):  LWE challenge with a sample BKW solver.
- [Handout](./handout): CI/CD will automatically build sites with `mdbook`,
    so you find it [here](https://sustcsc25.benx.dev).

## Description

Years ago, quantum computers were just a theoretical concept, but now they are becoming a reality. 
The devil quantum computers can break most of the cryptographic systems we use today, compromising
our data security. To fight against it, people rise up and design new cryptographic systems that are
resistant to quantum attacks, one of which is the Learning with Errors (LWE) problem. The LWE problem
is to find a secret vector $\mathbf{s} \in \mathbb{Z}_q^n$ given a set of noisy linear equations of 
the form

$$
\mathbf{A} \cdot \mathbf{s} + e = b \mod q
$$

where $\mathbf{A} \in \mathbb{Z}_q^{m \times n}$ is a matrix, $e$ is a noise vector, and $b$ is the result 
vector. We call $\mathbf{A}$ the lattice basis, $e$ the noise, and $b$ the ciphertext. The LWE problem is
hard to solve when the noise is small compared to the modulus $q$. This problem is currently a NP-hard
problem, and it is believed to be secure against quantum attacks.

Agent rustaceans, the time has come to prove your worth in the world of cryptography. Break LWE with your
excellent Rust skills and help improve the security of our systems.

Your challenge, should you choose to accept it, is to implement a solver for the LWE problem in Rust.

To evaluate the performance of your code, we will use the following metrics:
- Solving given LWE problem(s) with a upper bound of computation time $T$. You will get as many score as
  how far you can go within this limit.
- You should output a vector $\hat{s} \in \mathbb{Z}_q^n$ that is exactly the secret vector $\mathbf{s}$.
- If you finish all the tasks in the given time, the faster the better.

```rust
pub(crate) fn solve_lwe(
    n: usize,
    m: usize,
    q: u64,
    alpha: f64,
    a: &Array2<u64>,
    b: &Array1<u64>,
) -> Array1<u64> {
    Array1::zeros(n) // make a dummy guess
}
```

To accomplish this task, you can either by tuning the existing BKW solver or implementing a new one.
It will be super cool if you did both.

## Environment

[![](https://img.shields.io/badge/Rust-1.87-red?style=flat&logo=rust)](https://www.rust-lang.org)

> [!NOTE]
> Special note: 
> if you require nightly features and have a demand for changes in the evaluation environment, LMK.
> Latest changes in environment will be announced in the official group. Please make sure when
> you submit a `Dockerfile` that contains your required environment, otherwise we will run it with 
> the default `Dockerfile` in this repository.

See https://sustcsc25.benx.dev/setup/00-overview.html for setup instructions.

If having trouble with the setup or machine, please contact us.

## Rules and Benchmarking

You can only modify the `src/solver.rs` file, which contains the function `solve_lwe` that you need to implement. 
Any helper functions or modules can be added, but the signature of public function must remain the same. Any external
crates are allowed, but you should not use any crates including FFI or binding.

We'll be testing your code with 
- the clusters on [SUSTech's HPC platform](https://hpc.sustech.edu.cn/), with a single node and Intel 
  Xeon Platinum 2680-v3 (24 core)/6148 (40 core), no GPU or other accelerators.
- Maximum runtime is $T = 30$ minutes. Any exceeding runtime will be considered as a failure.

### Compilation

Your code is compiled with `release` profile under
- `opt-level = 1`
- `-C target-cpu=native`

If you prefer a nightly build, please state it clearly at the documentation (README or report).

### Forbidden

> [!WARNING]
> Keep your unsafe code minimal.

You should not
- output a constant or random vector
- use any external help using command line, but FFI is allowed
    - if you do so, your code may failed to compile because of the environment
    - ensure that you update the `Cargo.toml` and (`Dockerfile` or document)

You can do the following
- insert inline assembly code, but make sure it is supported by the target CPU

### Test Cases and Grading (87%)

| Test Case | n  | m    | q     | α      | Score |
|-----------|----|------|-------|--------|-------|
| 0         | 10 | 100  | 97    | 0.005  | 2     |
| 1         | 20 | 400  | 193   | 0.005  | 3     |
| 2         | 30 | 900  | 389   | 0.005  | 5     |
| 3         | 40 | 1500 | 769   | 0.005  | 7     |
| 4         | 45 | 1700 | 12289 | 0.005  | 9     |
| 5         | 50 | 2500 | 1543  | 0.005  | 11    |
| 6         | 55 | 3600 | 6151  | 0.005  | 13    |
| 7         | 30 | 1000 | 3079  | 0.010  | 17    |
| 8         | 40 | 1500 | 6151  | 0.010  | 19    |
|           |    |      |       | Total  | 86    |

For each case, error is calculated as

$$
\text{error} = \sum_{i=0}^{n-1} \left| s_i - \hat{s}_i \right| = 0
$$

### Report (14%)

Your report, in English or Chinese, should be a PDF file compiled by $\LaTeX$, markdown,
Typst or any other format that generates PDF. And it may include:

- Your optimization strategy, e.g., algorithm improvements, hardware features, etc.
- Performance analysis, e.g., profiling, flamegraph, etc.

You should clearly state references of the documents you read including, Academic papers,
Wikipedia, Blog posts and etc.

Your report is evaluated by the following rubrics:
- **Innovation (5 pt)**: Adapt code from existing libraries, or come up with new ideas.
    - Plagiarism, i.e., copying without citation -> 0 pt.
    - Improvement or implementation of algorithms -> 1 - 4 pt.
    - New techniques or algorithm (it's really cool) -> 5 pt.
- **Expression (5 pt)**: The report is concise and intuitive. It doesn't have to be long, but it should be clear.
    - Full of nonsense and errors -> 0 pt.
    - Rich in content, but hard to understand -> 1 - 4 pt.
    - Comprehensive and concise -> 5 pt.
- **Illustration (4 pt)**: The report contains figures, tables, or other illustrations to help explain your ideas.
    - No figures, and full of text -> 0 pt.
    - Some figures, but mostly referred from other sources or poorly designed -> 1 - 3 pt.
    - Figures are well-designed by yourself and help explain your ideas -> 4 pt.

## Submission

You have to submit your code along with a report by
- Compressed file, either `tar`, `gzip`, `zip`, or `7z`.
- A Link to your public repository, if you forked this repository.

Submitted files may look like
```
/sustcsc25-rs-<your teamid>
├── Cargo.lock
├── Cargo.toml
├── Dockerfile    # Optional, if you have a custom environment
├── src           # You can add any code here
│   ├── lwe.rs    # Encrypt Oracle
│   ├── solver.rs # Your solver
│   └── main.rs   # We ignore your main.rs since its judge
├── README.md     # Tell us about how to run your code
└── report.pdf
```

Do not compress the outer directory.
Rename your compressed file to `sustcsc25-rs-<your teamid>.*` with `<your teamid>` being your team ID
and `*` being the file extension of your compressed file.

### During the Contest

We only evaluate the performance (87% of the score) in this period,
which will be executed on the HPC platform every Tuesday and Friday.
So the report is not required during the contest period and will discarded.
Result and ranking are announced at the official group/website.

To submit your code, please send an email to `chanben04gz [AT] gmail.com` with the subject
`[SUSTCSC 25] Submission - <your teamid>` and attach your code. You can submit multiple times
before the evaluation, but only the latest submission will be considered.

> [!NOTE]
> If your submission is failed to compile or run, we will not give any feedback or score.

### Final Submission

Submit your code and report to the same email address with the subject
`[SUSTCSC 25] Final Submission - <your teamid>` with compressed file named
`sustcsc25-rs-final-<your teamid>.*`, before the ending of the contest period.
Multiple submissions are allowed, but only the latest submission will be accepted.

If you have integrity concerns, please also includes a checksum of the compressed files
in the email with your checksum algorithm, e.g., `CRC32`, `SHA256`, `MD5`, etc. This is optional,
and we will check that for you. Any unmatched checksum will be notified and disgarded.

## Hints

See https://sustcsc25.benx.dev/lwe/01-hints.html for optimization advices.

For crash course on Rust, see https://sustcsc25.benx.dev/rustup/00-first-look.html.

## Reference

- [Numerical Computing with Rust on CPU](https://plmlab.math.cnrs.fr/grasland/numerical-rust-cpu)
- [LWE Challenge by TU Darmstadt](https://www.latticechallenge.org/lwe_challenge/challenge.php)