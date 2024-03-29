//! Implementation of HyperLogLog
//!
//! [1]: https://algo.inria.fr/flajolet/Publications/FlFuGaMe07.pdf
//! [2]: https://en.wikipedia.org/wiki/HyperLogLog

use std::cmp;
use std::collections::HashSet;
use std::hash::Hash;

use clap::Parser;
use indicatif::{ProgressIterator, ProgressFinish};
use rand::Rng;

#[derive(Debug)]
struct HyperLogLog {
    register: Vec<u8>,
    index_bits: u8
}

impl Default for HyperLogLog {
    /// Creates a HyperLogLog with 4 bits as `index_bits`
    fn default() -> Self {
        Self::new(4).unwrap()
    }
}

/// Create a `HyperLogLog` with a number of index bits
macro_rules! HLL {
    ($index_bits:expr) => {
        HyperLogLog::new($index_bits).unwrap()
    };
}

impl HyperLogLog {
    /// Create a new HyperLogLog(HLL) set with first `index_bits` used as register indexes
    fn new(index_bits: u8) -> Result<Self, String> {
        if !(4..=16).contains(&index_bits) {
            return Err(
                format!(
                    "Number of index bits must be more than 0 and less than 9 (was {})", index_bits
                )
            );
        }
        let m: usize = helpers::registers_from_bits(&index_bits);
        Ok(Self { register: vec![0; m], index_bits })
    }

    /// Add a new hashable element to the set
    fn add<T: Hash>(&mut self, value: &T) {
        let hash = helpers::hash_value_32(value);
        let register_index: usize =
            helpers::n_be_bits(&hash, &(self.index_bits as u32))
            .try_into()
            .unwrap();
        // Count trailing zeros in remaining bits
        let non_index = helpers::n_le_bits(&hash, &(32 - self.index_bits as u32));
        let zeros: u8 = non_index.trailing_zeros() as u8 + 1;
        self.register[register_index] = cmp::max(zeros, self.register[register_index]);
    }

    /// Estimate `alpha`
    fn alpha(&self) -> f64 {
        let m: f64 = self.register.len() as f64;
        // Added ranges of values if register lenght happens to not be a power of 2
        if m < 32.0 {
            0.673
        } else if m < 64.0 {
            0.697
        } else if m < 128.0 {
            0.709
        } else {
            0.7213 / (1.0 + 1.079 / m)
        }
    }

    /// Count the cardinality of the current set
    fn count(&self) -> f64 {
        let alpha: f64 = self.alpha();
        let m_pow_2: f64 = self.register.len().pow(2) as f64;
        let indicator: f64 = helpers::indicator(&self.register);
        alpha * m_pow_2 * indicator
    }
}

mod helpers {
    use std::hash::Hash;

    use hash32::Hasher;

    /// Return a 32 bit hash of a `value`
    pub fn hash_value_32<T: Hash>(value: &T) -> u32 {
        let mut hasher = hash32::Murmur3Hasher::default();
        value.hash(&mut hasher);
        let hash: u32 = hasher.finish32();
        hash
    }

    /// Return `n` big endian (most significant) bits of a `value`
    pub fn n_be_bits(value: &u32, n: &u32) -> u32 {
        let shift_amount = 32 - n;
        value >> shift_amount
    }

    /// Return `n` least endian bits of a `value`
    pub fn n_le_bits(value: &u32, n: &u32) -> u32 {
        let bitmask: u32 = (1 << n) - 1;
        value & bitmask
    }

    /// Calculate number of registers based on `index_bits`
    pub fn registers_from_bits(index_bits: &u8) -> usize {
        2_usize.checked_pow(*index_bits as u32).unwrap()
    }

    /// Calculate indicator Z
    pub fn indicator(register: &[u8]) -> f64 {
        let val: f64 = register
            .iter()
            .map(|x| 1_f64 / 2_f64.powi(*x as i32))
            .sum();
        1_f64 / val
    }
}

#[derive(Parser)]
struct Cli {
    /// Number of index bits to use in HLL
    #[arg(default_value_t = 6)]
    index_bits: u8,
    /// Exponent of 10 for numbers to randomize
    #[arg(default_value_t = 8)]
    numbers_exp: u32,
    /// Exponent of 10 for maximum randomized number
    #[arg(default_value_t = 12)]
    max_value_exp: u32
}

fn main() {
    let args = Cli::parse();

    let numbers: usize = 10_usize.pow(args.numbers_exp);
    let min: usize = 0;
    let max: usize = 10_usize.pow(args.max_value_exp);

    let mut generator = rand::thread_rng();
    let mut hll = HLL!(8);
    let mut test_set: HashSet<usize> = HashSet::new();

    let bar_style = indicatif::ProgressStyle::with_template(
        "{bar:50} {pos}/{len} ETA: {eta_precise} Elapsed: {elapsed_precise}"
    ).unwrap();

    for _ in (0..numbers).progress().with_style(bar_style).with_finish(ProgressFinish::AndLeave) {
        let val = generator.gen_range(min..=max);
        hll.add(&val);
        test_set.insert(val);
    }
    let estimation = hll.count();
    let correct = test_set.len();
    let correct_f64 = correct as f64;
    let error: f64 = (estimation - correct_f64).abs() / correct_f64;
    println!("Cardinatity estimated with HashSet lenght\n> {:}", correct);
    println!("Cardinatity estimated with HLL\n> {:.2}", estimation);
    println!("Error\n> {:.2}%", error * 100.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registers_from_bits() {
        assert_eq!(helpers::registers_from_bits(&3), 8);
    }

    #[test]
    fn test_n_be_bits() {
        let number: u32 = 0b1010_0100_0000_0000_0000_0000_0000_0000;
        let ret = helpers::n_be_bits(&number, &6);
        assert_eq!(ret, 0b101001);
    }

    #[test]
    fn test_n_le_bits() {
        let number: u32 = 0b1010_0100;
        let ret = helpers::n_le_bits(&number, &3);
        assert_eq!(ret, 0b100);
    }

    #[test]
    fn test_hll_add() {
        let mut hll = HyperLogLog::new(4).unwrap();
        // Hash should equal 2766284370 = 10100100111000100010011001010010
        hll.add(&"moros".to_string());
        assert_eq!(hll.register, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0])
    }

    #[test]
    fn test_hll_macro() {
        let hll: HyperLogLog = HLL!(6);
        assert_eq!(hll.index_bits, 6);
    }
}
