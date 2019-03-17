fn extgcd(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
    let mut d = a;
    if b == 0 {
        *x = 1;
        *y = 0;
    } else {
        d = extgcd(b, a % b, y, x);
        *y -= (a / b) * (*x);
    }
    return d;
}

fn mod_inverse(a: i64, m: i64) -> i64 {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    extgcd(a, m, &mut x, &mut y);
    (m + x % m) % m
}

struct ModFactorial {
    fact: Vec<i64>,
}

// returns n! under mod m
impl ModFactorial {
    fn new(n: i64, m: i64) -> ModFactorial {
        let mut fact = vec![1; n as usize + 1];
        for i in 2..fact.len() {
            fact[i] = (i as i64 * fact[i - 1]) % m;
        }
        ModFactorial { fact: fact }
    }
}

// returns bin[i] = nCi under mod m
struct ModBinominal {
    bin: Vec<i64>,
}

impl ModBinominal {
    fn new(n: i64, m: i64, mod_factorial: &ModFactorial) -> ModBinominal {
        assert!(mod_factorial.fact.len() >= n as usize);
        let mut bin = vec![1; n as usize + 1];
        if n == 0 {
            return ModBinominal { bin: bin };
        }
        for i in 1..n + 1 {
            let a2 = mod_factorial.fact[i as usize] % m;
            let a3 = mod_factorial.fact[(n - i) as usize] % m;
            bin[i as usize] = mod_factorial.fact[n as usize] * mod_inverse(a2 * a3 % m, m) % m;
        }
        ModBinominal { bin: bin }
    }
}

fn h_comb(n: i64, r: i64, bins: &Vec<ModBinominal>) -> i64 {
    // nHr = n + r - 1Cr
    bins[(n + r - 1) as usize].bin[r as usize]
}

fn main() {
    let big_int = 1000_000_007i64;
    let fact7 = ModFactorial::new(7, big_int);
    assert_eq!(fact7.fact[0], 1);
    assert_eq!(fact7.fact[1], 1);
    assert_eq!(fact7.fact[2], 2);
    assert_eq!(fact7.fact[3], 6);
    assert_eq!(fact7.fact[7], 5040);

    let binom7 = ModBinominal::new(7, big_int, &fact7);
    assert_eq!(binom7.bin[0], 1);
    assert_eq!(binom7.bin[1], 7);
    assert_eq!(binom7.bin[2], 21);
    assert_eq!(binom7.bin[3], 35);
    assert_eq!(binom7.bin[7], 1);

    let fact14 = ModFactorial::new(14, big_int);
    let mut bins = Vec::with_capacity(15);
    for i in 0..15 {
        bins.push(ModBinominal::new(i, big_int, &fact14));
    }
    assert_eq!(h_comb(3, 4, &bins), 15);
}
