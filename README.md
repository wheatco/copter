# WheatCopter Flight Control

This code runs on the Raspberry Pi 2, receives sensor and control data and controls the motors.

## Flight controller firmware

_Note:_ This currently works with Mac OSX 10.11.4.

We're using Cleanflight on a Naze 32 for this project. Cleanflight is located in the `fc` subdirectory of this repository as a git submodule. We also have large files in the repository stored with `git lfs`, so you'll need to install that: `brew install git-lfs`.

The process for cloning the repo looks like this:

    git clone --recursive git@github.com:wheatco/copter.git
    git submodule foreach git pull origin master

Then, when the repo is open, just type `./build_fc.sh` to run the build script. It will automatically untar the cross compilation scripts if you haven't done so already, then use them to compile cleanflight (located in the `fc` directory). Also note that we're purposely using GCC v4.9 from Q2 2015 because newer versions are incompatible with the cleanflight.

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

## Plan
6th Week:

7th Week:

8th Week:

9th Week:

## Vision setup

1. Expand volume by running `raspi-config` on the pi.
2. `sudo apt-get install -y  build-essential cmake pkg-config git libjpeg8-dev libjasper-dev libpng12-dev libgtk2.0-dev libavcodec-dev libavformat-dev libswscale-dev libv4l-dev libatlas-base-dev gfortran python-pip python-dev `
3. `sudo apt-get install -f` if (2) fails
