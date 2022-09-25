
// n ^ p mod m
fn pow_m(n: i64, mut p: i64, m: i64) -> i64 {
    let mut r = n;
    let mut ret = 1;
    while p > 0 {
        if p % 2 == 0 {
            r = r * r % m;
            p /= 2;
        } else {
            ret = ret * r % m;
            p -= 1;
        }
    }
    ret
}

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

// return Y s.t. x^k ≡ y (mod m)
// k = sq * a + b
fn baby_step_giant_step(mut x: i64, mut y: i64, m: i64) -> i64 {
    x %= m;
    if gcd(x, m) != 1 {
        return -1;
    }
    let sq = (m as f64).sqrt().ceil() as i64;
    let mut mul = HashMap::new();
    let mut z = 1;

    // baby-step
    for b in 1..=sq {
        z = (z * x) % m;
        if !mul.contains_key(&z) {
            mul.insert(z, b);
        }
    }

    // giant-step
    let r = mod_inverse(pow_m(x, sq, m), m); // r = x^(-sq)
    for a in 0..=sq {
        if mul.contains_key(&y) {
            return sq * a + mul[&y];
        }
        y = (y * r) % m;
    }
    -1
}
