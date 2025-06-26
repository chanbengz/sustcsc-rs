# Running Rust on macOS

macOS is a Unix-like operating system, so it's similar to Linux in many ways.

## Package Manager: Homebrew

Though macOS has fantatstic GUI, developers will always need a package manager
in CLI to help them install and manage software - [Homebrew](https://brew.sh) 
is the most choice. [^1]

```bash
xcode-select --install
# This will install the Xcode Command Line Tools, which includes the necessary tools
# Type in your password if prompted
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

## Installing Rust

Getting libraries and Rust toolchain is easy with Homebrew.

```bash
brew install rustup curl pkgconf unzip util-linux wget && \
. "$HOME/.cargo/env" && \
rustup default stable # or nightly
```

Verify the installation by running:

```bash
rustc --version
```

We highly recommend you to have a VS Code and install the rust-analyzer extension.

[^1]: If you prefer to use MacPorts or Nix, skip this section plz.