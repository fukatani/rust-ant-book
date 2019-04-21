fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

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
    fn new(n: i64, m: i64, fact: &Vec<i64>, inv_fact: &Vec<i64>) -> ModBinominal {
        let mut bin = vec![1; n as usize + 1];
        if n == 0 {
            return ModBinominal { bin: bin };
        }
        for i in 1..n + 1 {
            let a2 = inv_fact[i as usize] % m;
            let a3 = inv_fact[(n - i) as usize] % m;
            bin[i as usize] = fact[n as usize] * ((a2 * a3) % m) % m;
        }
        ModBinominal { bin: bin }
    }
}

fn create_mod_bins(n: i64, big_int: i64) -> Vec<Vec<i64>> {
    // bins[n][i] = nCi
    let fact = ModFactorial::new(n, big_int).fact;
    let inv_fact = (0..fact.len())
        .map(|x| mod_inverse(fact[x as usize], big_int))
        .collect::<Vec<_>>();
    let mut bins = Vec::with_capacity(n as usize + 1);
    for i in 0..n + 1 {
        bins.push(ModBinominal::new(i, big_int, &fact, &inv_fact).bin);
    }
    bins
}

fn h_comb(n: i64, r: i64, bins: &Vec<Vec<i64>>) -> i64 {
    // nHr = n + r - 1Cr
    bins[(n + r - 1) as usize][r as usize]
}

fn main() {
    let big_int = 1000_000_007i64;
    let fact7 = ModFactorial::new(7, big_int);
    assert_eq!(fact7.fact[0], 1);
    assert_eq!(fact7.fact[1], 1);
    assert_eq!(fact7.fact[2], 2);
    assert_eq!(fact7.fact[3], 6);
    assert_eq!(fact7.fact[7], 5040);

    let fact7 = fact7.fact;
    let inv_fact7 = (0..fact7.len())
        .map(|x| mod_inverse(fact7[x as usize], big_int))
        .collect::<Vec<_>>();
    let binom7 = ModBinominal::new(7, big_int, &fact7, &inv_fact7);
    assert_eq!(binom7.bin[0], 1);
    assert_eq!(binom7.bin[1], 7);
    assert_eq!(binom7.bin[2], 21);
    assert_eq!(binom7.bin[3], 35);
    assert_eq!(binom7.bin[7], 1);

    let bins = create_mod_bins(8, big_int);
    assert_eq!(bins[7][3], 35);
    assert_eq!(bins[8][2], 28);

    // test h_comb
    assert_eq!(h_comb(3, 4, &bins), 15);
}
