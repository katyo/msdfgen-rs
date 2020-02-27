#!/bin/sh

crate=msdfgen-sys
bindings=$crate/src/darwin
targets="i686-apple-darwin x86_64-apple-darwin"

export PATH="/usr/local/opt/llvm/bin:$PATH"

cd $crate
#cargo clean

for target in $targets; do
    cargo build --features generate-bindings --target $target
done

cd ..

mkdir -p $bindings
rm -f $bindings/*.rs

for target in $targets; do
    cp target/$target/debug/build/$crate-*/out/bindings.rs $bindings/$(echo $target | sed 's/-apple-darwin//' | sed 's/i686/x86/').rs
done
