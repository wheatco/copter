# WheatCopter Flight Control

This code runs on the Raspberry Pi 2, receives sensor and control data and controls the motors.

## Setting Up Rust on your Pi

First, download [ARMv7](https://github.com/warricksothr/RustBuild#stable) builds of Cargo and Rust to `cargo.tar.gz` and `rust.tar.gz` on your Pi. Then just run:

    tar xzf cargo.tar.gz
    sudo tar xzf cargo.tar.gz
    sudo tar xzf rust.tar.gz
    chown -R root:users /opt/rust/nightly
    sudo chown -R root:users /opt/rust/nightly
    sudo chmod -R 775 /opt/rust/nightly
    sudo apt-get install git
    curl -sf https://raw.githubusercontent.com/brson/multirust/master/quick-install.sh | sh
    multirust update stable --link-local /opt/rust/stable
    multirust update unofficial-stable --link-local /opt/rust/stable/
    multirust default unofficial-stable

You should now have a running `rustc` and `cargo`.

## The Hardware

TODO add hardware info

## Compiling the Project

Make sure i2c is enabled. We did this by running `raspi-config` and choosing the enable i2c option.

Download this project to the Pi, and just `cargo run`.
