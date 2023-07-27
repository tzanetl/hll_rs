use std::hash::Hash;

struct HyperLogLog {
    register: Vec<u32>,
    index_bits: u8
}

impl Default for HyperLogLog {
    /// Creates a HyperLogLog with 3 bits as `index_bits`
    fn default() -> Self {
        Self::new(3).unwrap()
    }
}

impl HyperLogLog {
    /// Create a new HyperLogLog(HLL) set with first `index_bits` used as register indexes
    fn new(index_bits: u8) -> Result<Self, String> {
        if index_bits > 8 || index_bits < 1 {
            return Err(
                format!(
                    "Number of index bits must be more than 0 and less than 9 (was {})", index_bits
                )
            );
        }
        let m: usize = helpers::registers_from_bits(&index_bits);
        Ok(Self { register: vec![0; m as usize], index_bits })
    }

    /// Count the number of registers based on used `index_bits`
    fn registers(&self) -> usize {
        helpers::registers_from_bits(&self.index_bits)
    }

    /// Add a new hashable element to the set
    fn add<T: Hash>(mut self, value: &T) {
        todo!()
    }

    /// Count the cardinality of the current set
    fn count(self) -> usize {
        todo!()
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
        return hash;
    }

    /// Return `n` big endian (most significant) bits of a `value`
    pub fn n_be_bits(value: u32, n: u32) -> u32 {
        let shift_amount = 32 - n;
        value >> shift_amount
    }

    /// Calculate number of registers based on `index_bits`
    pub fn registers_from_bits(index_bits: &u8) -> usize {
        2_usize.checked_pow(*index_bits as u32).unwrap()
    } 
}

fn main() {
    println!("Hello, world!");
}
