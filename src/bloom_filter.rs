use std::arch::aarch64::uint64x2x4_t;
use std::hash::Hash;
use std::io::{Read, Write};
use crypto_hash::{Algorithm, digest, hex_digest};
use num_bigint::{BigUint, Sign};
use std::u32;

#[derive(Debug)]
pub struct BloomFilter {
    hash_functions: u8,
    size: u32,
    bitset: Vec<bool>,
}

impl BloomFilter {
    const ALGORITHMS: [(Algorithm, u32); 4] =
        [(Algorithm::MD5, 16), (Algorithm::SHA1, 20), (Algorithm::SHA256, 32), (Algorithm::SHA512, 64)];


    pub fn new(hash_functions: u8, size: u32) -> Result<Self, &'static str> {
        if (hash_functions > 4) {
            return Err("More than 4 hash functions not supported");
        }

        let bitset = vec![false; size as usize];
        let result = Ok(Self { hash_functions, size, bitset });
        return result;
    }

    pub fn insert(&mut self, element: &str) {
        let mut count = 0;
        for algo in Self::ALGORITHMS {
            let mut digest = digest(algo.0, element.as_bytes());

            let valBigInt = BigUint::from_bytes_le(digest.as_slice());
            let val = valBigInt.to_u32_digits();
            self.bitset[(val[0] % self.size) as usize] = true;
            count += 1;
            if (count == self.hash_functions) {
                break;
            }
        }
    }

    pub fn search(&self, element: &str) -> String {
        let mut result_msg = "Maybe";
        let mut count = 0;
        for algo in Self::ALGORITHMS {
            let mut digest = digest(algo.0, element.as_bytes());

            let valBigInt = BigUint::from_bytes_le(digest.as_slice());
            let val = valBigInt.to_u32_digits();
            if self.bitset[(val[0] % self.size) as usize] == false {
                result_msg = "Not present";
                break;
            }
            count += 1;
            if (count == self.hash_functions) {
                break;
            }
        }

        return String::from(result_msg);
    }
}
