use std::num::Wrapping;

const BASE: Wrapping<u64> = Wrapping(1009);
const MOD: Wrapping<u64> = Wrapping(1e9 as u64 + 7);

pub struct RollingHash {
    hash: Vec<Wrapping<u64>>,
    pow: Vec<Wrapping<u64>>,
}

impl RollingHash {
    pub fn new(s: &[Wrapping<u64>], base: Wrapping<u64>) -> RollingHash {
        let n = s.len();
        let mut hash: Vec<Wrapping<u64>> = vec![Wrapping(0); n + 1];
        let mut pow: Vec<Wrapping<u64>> = vec![Wrapping(0); n + 1];
        pow[0] = Wrapping(1);
        for i in 0..n {
            pow[i + 1] = (pow[i] * base) % MOD;
            hash[i + 1] = (hash[i] * base + s[i]) % MOD;
        }
        RollingHash {
            hash: hash,
            pow: pow,
        }
    }

    /// Get hash of [l, r)
    pub fn get_hash(&self, l: usize, r: usize) -> Wrapping<u64> {
        (self.hash[r] + MOD - (self.hash[l] * self.pow[r - l]) % MOD) % MOD
    }
}
