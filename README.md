# xorfiles_simd

A Rust command-line utility for XORing two files using SIMD acceleration.

The implementation uses Rust's `std::simd` API with runtime CPU feature
detection:

1. AVX-512 path, when available
2. AVX2 fallback, when AVX-512 is not available
3. Scalar fallback, when neither AVX-512 nor AVX2 is available

The two input files must have exactly the same size. The output file will contain
the byte-wise XOR result.

## Features

- Byte-wise XOR of two files
- Runtime dispatch:
  - AVX-512 when supported by the CPU
  - AVX2 fallback
  - Portable scalar fallback
- Uses Rust `std::simd` instead of directly writing AVX2 intrinsics
- Supports arbitrary binary file formats
- Simple command-line interface

## Requirements

This project uses Rust's experimental `std::simd` API, so it currently requires
the Rust nightly toolchain.

Install nightly with:

```bash
rustup toolchain install nightly
```

Recommended project-local toolchain configuration:

```toml
# rust-toolchain.toml
[toolchain]
channel = "nightly"
```

The source must also enable portable SIMD:

```rust
#![feature(portable_simd)]
```

## Installation

Clone the repository:

```bash
git clone https://github.com/qnfm/xorfiles_simd.git
cd xorfiles_simd
```

Build with nightly:

```bash
cargo +nightly build --release
```

If `rust-toolchain.toml` is present in the project root, this also works:

```bash
cargo build --release
```

## Usage

```bash
./target/release/xorfiles_simd <input_file1> <input_file2> <output_file>
```

Example:

```bash
./target/release/xorfiles_simd file1.bin file2.bin output.bin
```

Both input files must have exactly the same size.

The output file is computed byte by byte:

```text
output[i] = input_file1[i] XOR input_file2[i]
```

For example, if:

```text
input_file1[i] = 0b10101010
input_file2[i] = 0b11001100
```

then:

```text
output[i] = 0b01100110
```

## Runtime SIMD dispatch

The program checks CPU features at runtime.

On x86/x86_64 targets, the dispatch order is:

```text
AVX-512 -> AVX2 -> scalar
```

That means the same binary can run on different machines and automatically use
the best available implementation.

The SIMD paths are selected with:

```rust
std::is_x86_feature_detected!("avx512f")
std::is_x86_feature_detected!("avx2")
```

The AVX-512 path processes 64 bytes per vector.

The AVX2 path processes 32 bytes per vector.

Remaining bytes, if any, are handled by a scalar remainder loop.

## Troubleshooting

### `#![feature] may not be used on the stable release channel`

This means Cargo is using stable Rust.

Check the active toolchain:

```bash
rustup show active-toolchain
```

Build explicitly with nightly:

```bash
cargo +nightly build --release
```

Or add this file to the project root:

```toml
# rust-toolchain.toml
[toolchain]
channel = "nightly"
```

### `unresolved imports std::simd::LaneCount, std::simd::SupportedLaneCount`

Use the SIMD prelude instead:

```rust
use std::simd::prelude::*;
```

For fixed lane counts such as 32 and 64, an explicit `LaneCount` bound is not
needed in the simple XOR implementation.

## Contributing

Contributions are welcome. Please open an issue or submit a pull request for bug
fixes, performance improvements, or portability changes.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more
details.
