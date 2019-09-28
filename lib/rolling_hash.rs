const BASE: u64 = 1e9 as u64 + 7;
const MOD: u64 = 1e9 as u64 + 9;

pub struct RollingHash {
    hash: Vec<u64>,
    pow: Vec<u64>,
}

impl RollingHash {
    pub fn new(s: &[u64], base: u64) -> RollingHash {
        let n = s.len();
        let mut hash: Vec<u64> = vec![0; n + 1];
        let mut pow: Vec<u64> = vec![0; n + 1];
        pow[0] = 1;
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
    pub fn get_hash(&self, l: usize, r: usize) -> u64 {
        (self.hash[r] + MOD - (self.hash[l] * self.pow[r - l]) % MOD) % MOD
    }
}
