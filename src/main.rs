use std::hash::Hash;

use hash32;

struct HyperLogLog {
    register: Vec<u32>,
    index_bits: u8
}

impl Default for HyperLogLog {
    /// Creates a HyperLogLog with 4 bits as `index_bits`
    fn default() -> Self {
        Self::new(3).unwrap()
    }
}

impl HyperLogLog {
    /// Create a new HyperLogLog(HLL) set with first `index_bits` used as register indexes
    fn new(index_bits: u8) -> Result<Self, String> {
        if index_bits > 8 {
            return Err(format!("Number of index bits must be less than 9 (was {})", index_bits));
        }
        let m: usize = 2_usize.checked_pow(index_bits as u32).unwrap();
        Ok(Self { register: vec![0; m as usize], index_bits })
    }

    /// Add a new hashable element to the set
    fn add<T: Hash>(mut self, value: &T) {
        // let mut hasher = hash32::Murmur3Hasher::default();
        // value.hash(&mut hasher);
        // let hash: u32 = hasher.finish32();
        todo!()
    }

    /// Count the cardinality of the current set
    fn count(self) -> usize {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
