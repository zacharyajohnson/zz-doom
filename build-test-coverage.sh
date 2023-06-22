#!/bin/sh

cargo llvm-cov > /dev/null 2>&1
if [ $? -eq 0 ]; then
        printf "cargo llvm-cov already installed...\n"
else
        printf "Installing llvm-cov...\n"
        cargo +stable install cargo-llvm-cov --locked
fi

cargo llvm-cov --html
