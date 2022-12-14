[![Rust](https://github.com/Omnissi/NISTRS/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/Omnissi/NISTRS/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/Omnissi/NISTRS/branch/main/graph/badge.svg?token=gHuJhZ2bgc)](https://codecov.io/gh/Omnissi/NISTRS)
[![CodeFactor](https://www.codefactor.io/repository/github/omnissi/nistrs/badge)](https://www.codefactor.io/repository/github/omnissi/nistrs)
[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/Omnissi/NISTRS.svg)](https://isitmaintained.com/project/Omnissi/NISTRS "Average time to resolve an issue")
[![Percentage of issues still open](https://isitmaintained.com/badge/open/Omnissi/NISTRS.svg)](https://isitmaintained.com/project/Omnissi/NISTRS "Percentage of issues still open")

# nistrs

This crate implements statistical tests according to the [NIST standard](https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-22r1a.pdf).

## Example usage:

```rust
use nistrs::prelude::*;

let data = BitsData::from_binary(vec!(0x23, 0x44));
let result = frequency_test(&data);
print!("Test passed: {}; P-value: {}", result.0, result.1);
```
