#!/bin/bash
export PATH=$PATH:$(pwd)/gcc-arm-none-eabi-5_3-2016q1/bin
echo $PATH
cd fc
make clean TARGET=NAZE
make TARGET=NAZE
