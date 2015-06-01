#!/bin/sh

{ cargo build && cat ./target/debug/build/rust-cryptopp-*/output; } \
  || cat ./target/debug/build/rust-cryptopp-*/out/gen.{cpp,rs}

