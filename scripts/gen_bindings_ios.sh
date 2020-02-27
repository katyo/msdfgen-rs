#!/bin/sh

crate=msdfgen-sys
bindings=$crate/src/ios
targets="armv7-apple-ios aarch64-apple-ios i386-apple-ios x86_64-apple-ios"

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
    cp target/$target/debug/build/$crate-*/out/bindings.rs $bindings/$(echo $target | sed 's/-apple-ios//' | sed 's/i386/x86/').rs
done
