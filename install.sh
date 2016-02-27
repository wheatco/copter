#!/bin/bash
# Install Rust, multirust heavily recommended
sudo apt-get install -y -qq curl git
curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sudo sh << "y"

# Step 0: Our target is an ARMv7 device, the triple for this target is `arm-unknown-linux-gnueabihf`

# Step 1: Install the C cross toolchain
sudo apt-get install -y -qq gcc-arm-linux-gnueabihf

# Step 2: Install the cross compiled standard crates
# (NOTE In the future (*), all these steps will be replaced by a single command:
#  `multirust add-target arm-unknown-linux-gnueabihf`)
mkdir tmp
cd tmp
rustc -V
curl -O http://static.rust-lang.org/dist/rust-std-1.6.0-arm-unknown-linux-gnueabihf.tar.gz
tar xzf rust-std-1.6.0-arm-unknown-linux-gnueabihf.tar.gz
cd rust-std-1.6.0-arm-unknown-linux-gnueabihf
./install.sh --prefix=$(rustc --print sysroot)
cd ../..
rm -r tmp

# Step 3: Configure cargo for cross compilation
mkdir -p ~/.cargo
echo '[target.arm-unknown-linux-gnueabihf]' > ~/.cargo/config
echo 'linker = "arm-linux-gnueabihf-gcc"' > ~/.cargo/config

# # Test cross compiling a Cargo project
# cargo new --bin hello
# cd hello
# cargo build --target=arm-unknown-linux-gnueabihf
#    Compiling hello v0.1.0 (file:///home/ubuntu/hello)
# file target/arm-unknown-linux-gnueabihf/debug/hello
# hello: ELF 32-bit LSB  shared object, ARM, EABI5 version 1 (SYSV), dynamically linked (uses shared libs), for GNU/Linux 2.6.32, BuildID[sha1]=67b58f42db4842dafb8a15f8d47de87ca12cc7de, not stripped

# # Test the binary
# scp target/arm-unknown-linux-gnueabihf/debug/hello me@arm:~
# ssh me@arm:~ ./hello
# Hello, world!
