# Decoder for DXTn-compressed data
[![Build Status](https://travis-ci.org/ifeherva/decrunch.svg?branch=master)](https://travis-ci.org/ifeherva/decrunch) [![Crates.io](https://img.shields.io/crates/v/decrunch.svg)](https://crates.io/crates/decrunch)

This crate provides a Rust wrapper around [crunch's](https://github.com/BinomialLLC/crunch) decompressor.

[Documentation](https://docs.rs/decrunch/0.1.0/)

# Example

```rust
use decrunch::*;
use std::fs::File;
use std::io::Read;

# use std::io;
# fn foo() -> io::Result<()> {
let mut compressed_file = File::open("testdata/copyright_2048_compressed.dat")?;
let mut compressed_data = Vec::new();

compressed_file.read_to_end(&mut compressed_data)?;

let c_data = CrunchedData::new(&compressed_data);
let decompressed_data = match c_data.decode_level(0) {
    None => {
        panic!("Failed to decompress texture data");
    }
    Some(res) => res,
};

assert!(decompressed_data.len() > 0);

# Ok(())
# }
```
