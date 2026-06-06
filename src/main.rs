#![feature(portable_simd)]

use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::simd::prelude::*;

#[inline(always)]
fn xor_simd<const LANES: usize>(data1: &[u8], data2: &[u8]) -> Vec<u8> {
    assert_eq!(
        data1.len(),
        data2.len(),
        "Data slices must be of the same length."
    );

    let len = data1.len();
    let mut result = vec![0u8; len];

    let mut i = 0;

    while i + LANES <= len {
        let a = Simd::<u8, LANES>::from_slice(&data1[i..i + LANES]);
        let b = Simd::<u8, LANES>::from_slice(&data2[i..i + LANES]);
        let c = a ^ b;

        c.copy_to_slice(&mut result[i..i + LANES]);

        i += LANES;
    }

    for j in i..len {
        result[j] = data1[j] ^ data2[j];
    }

    result
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx512f")]
unsafe fn xor_avx512(data1: &[u8], data2: &[u8]) -> Vec<u8> {
    // 64 lanes × u8 = 512 bits
    xor_simd::<64>(data1, data2)
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn xor_avx2(data1: &[u8], data2: &[u8]) -> Vec<u8> {
    // 32 lanes × u8 = 256 bits
    xor_simd::<32>(data1, data2)
}

fn xor_scalar(data1: &[u8], data2: &[u8]) -> Vec<u8> {
    assert_eq!(
        data1.len(),
        data2.len(),
        "Data slices must be of the same length."
    );

    data1.iter().zip(data2.iter()).map(|(a, b)| a ^ b).collect()
}

fn xor_dispatch(data1: &[u8], data2: &[u8]) -> Vec<u8> {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if std::is_x86_feature_detected!("avx512f") {
            return unsafe { xor_avx512(data1, data2) };
        }

        if std::is_x86_feature_detected!("avx2") {
            return unsafe { xor_avx2(data1, data2) };
        }
    }

    xor_scalar(data1, data2)
}

fn read_file(path: &Path) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn write_output(path: &Path, data: &[u8]) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: xorfiles_simd <filepath1> <filepath2> <output>");
        std::process::exit(1);
    }

    let path1 = Path::new(&args[1]);
    let path2 = Path::new(&args[2]);
    let output_path = Path::new(&args[3]);

    let data1 = read_file(path1)?;
    let data2 = read_file(path2)?;

    if data1.len() != data2.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Files are not of the same size",
        ));
    }

    let result = xor_dispatch(&data1, &data2);

    write_output(output_path, &result)?;
    Ok(())
}
