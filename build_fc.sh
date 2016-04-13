#!/bin/bash
if [ ! -d gcc-arm-none-eabi-4_9-2015q2/bin ]; then
    tar -xzvf gcc-arm-none-eabi-4_9-2015q2-20150609-mac.tar.bz2
fi;

export PATH=$PATH:$(pwd)/gcc-arm-none-eabi-4_9-2015q2/bin
echo $PATH
cd fc
make clean TARGET=NAZE
make TARGET=NAZE
