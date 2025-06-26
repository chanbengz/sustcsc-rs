# Environment Setup

This challenge only focuses on optimization on CPU so that saves you a lot of time learning CUDA or OpenCL.
This chapter assumes you have a basic knowledge of computer and programming (e.g., if you learned some languages),
and you don't know anything about Rust. We will get you started with Rust on local bare-metal machine or container.

If you'd rather not spend too much time on setting up the environment, a Linux/macOS machine will be the best choice.
Otherwise, you could use a Docker container but it may take a while to install Docker and set up the network. Don't worry,
we will provide a Dockerfile for you and guideline to resolve the issues. The worst choice is to use Windows, 
but if you have to, you can enable WSL2 or install Docker Desktop.

We run your code in a Docker container after the submission and we may resolve some issues that is not regarding to your code.
However, if we decide it's a bug in your code, we will not fix that and will consider it as a failure. For performance issues,
we will ensure that your code is running in a native CPU environment, and for the worst case, we might turn to a grading scheme
by comparing all of submissions (i.e., relative performance).

A tip about Rust that it's associated with a package manager called Cargo and online repository called [crates.io](https://crates.io).
You can find documents of crates on [docs.rs](https://docs.rs).

## Overview of Benchmarking Environment

- CPU: Intel Xeon Platinum 8350C (2.40GHz * 20 core * 2 sockets)
- Rust: stable, possibly 1.87.0[^0]
- GPU: N/A (we won't use GPU in this challenge)
- Host OS: Linux 4.18.0-372.32.1.el8_6.x86_64, all mitigation disabled
- Network: N/A[^1]
- Docker: Singularity 3.7.1

[^0]: Signal us if you'd like to use a nightly version anywhere in the submission. However, you have to be responsible for the nightly version since it may break or slow down your code.

[^1]: We ignore all compilation cost, including the cost of downloading dependencies, and we will not run your code in a container with network access.