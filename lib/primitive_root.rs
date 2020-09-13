fn main() {
    let p = 11;
    let g = find_primitive_root(p as i64) as usize;

    let mut powers = vec![1];
    for _ in 1..p - 1 {
        let last = powers[powers.len() - 1];
        powers.push((last * g) % p);
    }
    // g ^ i = power[i];
    assert_eq!(powers, vec![1, 2, 4, 8, 5, 10, 9, 7, 3, 6]);

    let mut logs = vec![-1; p];
    for i in 0..p - 1 {
        logs[powers[i]] = i as i64;
    }
    // g ^ logs[i] = i (where i > 0)
    assert_eq!(logs, vec![-1, 0, 1, 8, 2, 4, 9, 7, 3, 6, 5]);
}

fn find_primitive_root(p: i64) -> i64 {
    let mut fact = vec![];
    let phi = p - 1;
    let mut n = phi;
    for i in 2.. {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            fact.push(i);
            while n % i == 0 {
                n /= i;
            }
        }
    }
    if n > 1 {
        fact.push(n);
    }

    'outer: for res in 2..p + 1 {
        let mut ok = true;
        for i in 0..fact.len() {
            ok &= pow_m(res, phi / fact[i], p) != 1;
            if !ok {
                continue 'outer;
            }
        }
        if ok {
            return res;
        }
    }
    unreachable!();
}

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
