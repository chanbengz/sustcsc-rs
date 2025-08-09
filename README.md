# SUSTCSC 2025 - Rust for HPC

> [!NOTE]
> Contact: Ben Chen \<chenb2022@mail.sustech.edu.cn\>
> 
> Platform: Xeon Platinum CPU

## Structure

In this repos you'll find:
- [Starter's Code](./challenge):  LWE-powered FHE challenge.
    - [bonus](./challenge/bonu): Bonus challenge, to crack the LWE problem
- [Handout](./handout): CI/CD will automatically build sites with `mdbook`,
    so you find it [here](https://sustcsc25.benx.dev).

## Crypto 101

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

Clever as you are, you can find that this scheme can be used to implement a Fully Homomorphic Encryption
(FHE) system. A FHE system allows you to perform computations on encrypted data without decrypting it, i.e.,

$$
A_e + B_e = C_e \neq (A + B)_e
$$

where $C = A + B$ but you cannot get $C_e$ from the encrypted sum of $A$ and $B$ directly [^1]. So, you
can give out your sensitive data and confidentially let the server perform computations on it, and the server
cannot learn anything about your data. This is the core idea of FHE, and it is a powerful tool for 
privacy-preserving computations.

[^1]: This is a Cryptography assumption, probably known as the CCA/CPA security.

## Fhe Game of Life

However, the current FHE systems will degrade computation throughput, and we will show you in this challenge. 

Conway's game of life is a cellular automaton devised by the British mathematician John Horton Conway in 1970.
When initialized with a random grid, it will evolve the grid according to the following rules:
- Any live cell with fewer than two live neighbours dies, as if caused by under-population.
- Any live cell with two or three live neighbours lives on to the next generation.
- Any live cell with more than three live neighbours dies, as if by over-population.
- Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

Our FHE system goes through the following steps:
1. (Client) Initialize a random grid of size $m \times n$ with each cell being either alive (1) or dead (0).
2. (Client) Encrypt the grid using the LWE scheme and send the ciphertext to the server.
3. (Server) Perform the game of life on the encrypted grid for a given number of steps.
4. (Client) Decrypt and verify the result.

In this challenge, you will improve this FHE system [^2] that can perform the game of life on encrypted data.
As the computation is costly, you will need to optimize the code and _compilation_ to get a better performance.

[^2]: By system, we mean the client and server code, and the FHE library.

## Environment

[![](https://img.shields.io/badge/Rust-1.87-orange?style=flat&logo=rust)](https://www.rust-lang.org)
[![](https://img.shields.io/badge/Rust-nightly-red?style=flat&logo=rust)](https://www.rust-lang.org)

> [!NOTE]
> Special note: 
> if you require nightly features and have a demand for changes in the evaluation environment, LMK.
> Latest changes in environment will be announced in the official group. Please make sure when
> you submit a `Dockerfile` that contains your required environment, otherwise we will run it with 
> the default `Dockerfile` in this repository.

See https://sustcsc25.benx.dev/setup/00-overview.html for setup instructions.

If having trouble with the setup or machine, please contact us.

## Rules and Benchmarking

Any code except for the judge code must be written in Rust, and you can use any libraries from crates.io.
You cannot touch the judge code, which is the `main.rs`, but anything else is free to go.

We'll be testing your code with the clusters on [SUSTech's HPC platform](https://hpc.sustech.edu.cn/) with
- a single node with Intel Xeon Xeon Platinum 8350C (2.40GHz * 20 core * 2 sockets)
- (or) a single node with Intel Xeon Platinum 8175M (2.5 GHz * 24 core * 2 sockets)
- no GPU or other accelerators.

### Compilation

Your code is compiled with default `release` profile under
- `opt-level = 1`
- `-C target-cpu=native` (should not be changed)

If you think tweaking compilation flags will improve the performance, do it.

If you prefer a nightly build, please state it clearly at the documentation (README or report).

### Forbidden

> [!WARNING]
> Keep your unsafe code minimal.

You should not
- remove the FHE encryption/decryption
- use any external help using command line, but FFI is allowed
    - if you do so, your code may failed to compile because of the environment
    - ensure that you update the `Cargo.toml` and (`Dockerfile` or document)

You can do the following
- insert inline assembly code, but make sure it is supported by the target CPU

### Test Cases and Grading (86%)

| Test Case | m  | n    | steps | time (s) | Score |
|-----------|----|------|-------|----------|-------|
| 0         | 3  | 3    | 1     | 7.0      | 2     |
| 1         | 5  | 5    | 1     | 10.0     | 3     |
| 2         | 5  | 5    | 2     | 15.0     | 5     |
| 3         | 7  | 7    | 2     | 21.0     | 7     |
| 4         | 7  | 7    | 4     | 39.0     | 9     |
| 5         | 8  | 10   | 4     | 70.0     | 11    |
| 6         | 10 | 12   | 4     | 96.0     | 13    |
| 7         | 15 | 15   | 4     | 128.0    | 17    |
| 8         | 17 | 17   | 5     | 196.0    | 19    |
|           |    |      |       | Total    | 86    |

For each test case, you will be given a random grid of size $m \times n$ and 
a number of steps to run the game of life. Every test case is considered passed if
and only if the `verify` function returns `true` under the given running time.

### Report (14%)

Your report, in English or Chinese, should be a PDF file compiled by $\LaTeX$, markdown,
Typst or any other format that generates PDF. And it may include:

- Your optimization strategy, e.g., algorithm improvements, hardware features, etc.
- Performance analysis, e.g., profiling, flamegraph, etc.

You should clearly state references of the documents you read including, Academic papers,
Wikipedia, Blog posts and etc.

> [!WARNING]
> Don't use too much AI-generated content, because we do read your report ourselves.
> The committee members are experienced in identifying AI-generated content,
> and if we find too much AI-generated content in your report, we will give zero to either
> of these rubrics below.

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

## Bonus: Breaking LWE (50%)

> [!NOTE]
> To get a rustacean award, you're suggested to complete this challenge;
> it might be a little bit tricky tho.

Agent rustaceans, the time has come to prove your worth in the world of cryptography. Break LWE with your
excellent Rust skills and help improve the security of our systems.

Your challenge, should you choose to accept it, is to implement a solver for the LWE problem in Rust.

To accomplish this task, you can either by tuning the existing solver or implementing a new one.
Your solution will be evaluated based on its performance and correctness.

| Test Case | n  | m    | q     | α      | Score |
|-----------|----|------|-------|--------|-------|
| 0         | 10 | 100  | 97    | 0.005  | 2     |
| 1         | 20 | 400  | 193   | 0.005  | 3     |
| 2         | 30 | 900  | 389   | 0.005  | 5     |
| 3         | 40 | 1500 | 769   | 0.005  | 7     |
| 4         | 45 | 1700 | 12289 | 0.005  | 9     |
| 5         | 50 | 2500 | 1543  | 0.005  | 11    |
| 6         | 55 | 3600 | 6151  | 0.005  | 13    |
|           |    |      |       | Total  | 50    |

For each case, error is calculated as follows.

$$
\text{error} = \sum_{i=0}^{n-1} \left| s_i - \hat{s}_i \right| = 0
$$

This part will be run within 30 mins. The bonus points directly go to your final score.

## Submission

You have to submit your code along with a report by
- Compressed file, either `tar`, `gzip`, `zip`, or `7z`.
- A Link to your public repository, if you forked this repository.

Submitted files may look like
```
/sustcsc25-rs-<your teamid>
├── Cargo.lock
├── Cargo.toml    # Cargo manifest *
├── Dockerfile    # Optional, if you have a custom environment
├── bonus         # Bonus challenge
│   ├── Cargo.toml
│   └── src
│       ├── solver.rs
│       └── main.rs
├── src           # You can add any code here
│   ├── client.rs # Client code *
│   ├── server.rs # Server code *
│   └── main.rs   # We ignore your main.rs since its judge
└── README.md     # Tell us about how to run your code
```

_* indicates you can modify this file_

See https://handicraft-computing-team.github.io/sustcsc-doc/pages/intro/basic.html

## Hints

See https://sustcsc25.benx.dev/lwe/01-hints.html for optimization advices.

For crash course on Rust, see https://sustcsc25.benx.dev/rustup/00-first-look.html.

## Reference Solution

> [!NOTE]
> 参考自出题时候的设想以及选手们的解法。

### Basic

> 本来是没有这题的, [主办方](https://github.com/Jaredanwolfgang)担心太难了所以将一开始的题目放到Bonus并出了这道"非常简单"的题目。看上去低估了我们的选手。
>
> 设计思路是没有的, 主要是让大家熟悉一下Rust。

思路是rayon(多线程) + tfhe-rs 自带的simd选项 + Rust编译优化, 还是非常straightforward。

坑点在于rayon的多线程问题, tfhe-rs有[说明](https://docs.zama.ai/tfhe-rs/fhe-computation/advanced-features/rayon-crate#working-example), 简单地考一下Rust多线程和borrow的知识。

因为benchmark baseline的机器性能比赛事用的机器性能稍弱, 所以题目要求放宽送了一些。预期解法还需要加上一些代码优化, 如替换数据类型(故意用的UInt8, 实际上Bool就够了), 减少密钥位数(因为会降低安全性不推荐, 但允许)

### Bonus

设计的时候想了无数种方案: 本来想出成CTF形式的(即魔改一个LWE让它变得trivial), 但Rust的数学库实在太少了, 对选手们不太友好, 遂放弃; 
一般来说出一个Rust科学计算最好, 比如Reference里面的Gray-scott reaction diffusion, 但本人数理基础很烂, 此类问题也不好量化成赛题类型, 遂放弃;
于是想到这个~~简单的线性代数问题, 只需上过线代+一点冲浪自学即可~~的题目, 参数也写的比较小。

~~初始代码能过第一个0分样例, 摆了先不写。~~

## Reference

- [tfhe-rs](https://github.com/zama-ai/tfhe-rs)
- [Numerical Computing with Rust on CPU](https://plmlab.math.cnrs.fr/grasland/numerical-rust-cpu)
- [LWE Challenge by TU Darmstadt](https://www.latticechallenge.org/lwe_challenge/challenge.php)
- [LLL implementation in Rust](https://github.com/murcoutinho/LLL)
- [Attacks on LWE](https://www.maths.ox.ac.uk/system/files/attachments/lattice-reduction-and-attacks.pdf)
