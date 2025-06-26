# Running Rust on Linux

Well, if you're using Linux, you know how to do it. 

But if you don't, here's a quick guide to get you started.

## Preparing the System packages

Linux is just a kernel and there're various distributions of Linux, which means that you may need to find out
what you got because they have different package managers, by executing

```bash
uname -a
# will output something like:
# Linux your-hostname 6.14.6-arch1-1 #1 SMP PREEMPT_DYNAMIC Fri, 09 May 2025 17:36:18 +0000 x86_64 GNU/Linux
cat /etc/os-release
# NAME="Arch Linux"
# PRETTY_NAME="Arch Linux"
# ID=arch
# BUILD_ID=rolling
# ANSI_COLOR="38;2;23;147;209"
# HOME_URL="https://archlinux.org/"
# DOCUMENTATION_URL="https://wiki.archlinux.org/"
# SUPPORT_URL="https://bbs.archlinux.org/"
# BUG_REPORT_URL="https://gitlab.archlinux.org/groups/archlinux/-/issues"
# PRIVACY_POLICY_URL="https://terms.archlinux.org/docs/privacy-policy/"
# LOGO=archlinux-logo
```

In this example, we are using Arch Linux. Once you know what distribution you're using, go on

### Debian/Ubuntu

```bash
sudo apt-get update && \
sudo apt-get install -y build-essential \
    ca-certificates curl \
    pkg-config unzip util-linux wget \

```

### Fedora

```bash
sudo dnf makecache --refresh && \
sudo dnf group install c-development && \
sudo dnf install ca-certificates curl \
    pkg-config unzip util-linux wget
```

### RHEL

```bash
sudo dnf makecache --refresh && \
sudo dnf groupinstall "Devlopment tools" && \
sudo dnf install epel-release && \
sudo /usr/bin/crb enable && \
sudo dnf makecache --refresh && \
sudo dnf install ca-certificates curl \
    pkg-config unzip util-linux wget
```

### Arch

```bash
sudo pacman -Syyu && \
sudo pacman -S base-devel ca-certificates curl  \
    pkg-config unzip util-linux wget
```

### openSUSE

```bash
sudo zypper ref && \
sudo zypper in -t pattern devel_C_C++ && \
sudo zypper in ca-certificates curl \
    pkg-config unzip util-linux wget
```

## Installing Rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && \
. "$HOME/.cargo/env" && \
rustup default stable # or nightly
```

Verify the installation by running

```bash
rustc --version
```

You're all set to go!