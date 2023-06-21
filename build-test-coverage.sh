#!/bin/sh

#rm -rf ./*.profraw
#rm -rf target/debug/coverage

#rustup component add llvm-tools-preview
cargo llvm-cov > /dev/null 2>&1
if [ $? -eq 0 ]; then
        printf "cargo llvm-cov already installed...\n"
else
        printf "Installing llvm-cov...\n"
        cargo +stable install cargo-llvm-cov --locked
fi

cargo llvm-cov --html

#if command -v grcov; then
#        printf "grcov already installed...\n"
#else
#        printf "Installing grcov....\n"
#        cargo install --force grcov
#fi

#export RUSTFLAGS="-Cinstrument-coverage"

#cargo build

#export LLVM_PROFILE_FILE="zz-doom-%p-%m.profraw"
#cargo test

#grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
