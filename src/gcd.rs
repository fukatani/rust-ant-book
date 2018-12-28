fn gcd(a: i32, b:i32) -> i32 {
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
    fact: Vec<i64>
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
    bin: Vec<i64>
}

impl ModBinominal {
    fn new(n: i64, m: i64) -> ModBinominal {
        let mod_factorial = ModFactorial::new(n, m);
        let mut bin = vec![1; n as usize + 1];
        for i in 1..mod_factorial.fact.len() - 1 {
            let a2 = mod_factorial.fact[i] % m;
            let a3 = mod_factorial.fact[n as usize - i] % m;
            bin[i] = mod_factorial.fact[n as usize] * mod_inverse(a2 * a3 % m, m) % m;
        }
        ModBinominal { bin: bin }
    }
}

fn main() {
    let n = 100i64;
    let bin = ModBinominal::new(n, 100_000_000 + 7);
    for i in 0..n as usize + 1 {
        println!("{:?}", bin.bin[i]);
    }
}
