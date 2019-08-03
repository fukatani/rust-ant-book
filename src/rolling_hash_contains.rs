use std::cmp::*;
use std::num::Wrapping;

const big_int: Wrapping<u64> = Wrapping(1000_000_000 + 7);

// b contains a?
fn contains(a: &Vec<u64>, b: &Vec<u64>) -> bool {
    if a.len() > b.len() {
        return false;
    }
    let a = a.iter().map(|&x| Wrapping(x)).collect::<Vec<_>>();
    let b = b.iter().map(|&x| Wrapping(x)).collect::<Vec<_>>();
    let mut power_base = Wrapping(1u64);
    for _ in 0..a.len() {
        power_base *= big_int;
    }
    let mut ah = Wrapping(0u64);
    let mut bh = Wrapping(0u64);
    for i in 0..a.len() {
        ah = ah * big_int + a[i];
    }
    for i in 0..a.len() {
        bh = bh * big_int + b[i];
    }

    for i in 0..a.len() {
        if ah == bh {
            return true;
        }
        if i + a.len() >= b.len() {
            break;
        }
        bh = bh * big_int + b[i + a.len()] - b[i] * power_base;
    }
    false
}

fn overlap(a: &Vec<u64>, b: &Vec<u64>) -> usize {
    let a = a.iter().map(|&x| Wrapping(x)).collect::<Vec<_>>();
    let b = b.iter().map(|&x| Wrapping(x)).collect::<Vec<_>>();

    let mut ans = 0;
    let mut ah = Wrapping(0u64);
    let mut bh = Wrapping(0u64);
    let mut t = Wrapping(1u64);
    for i in 1..min(a.len(), b.len()) + 1 {
        ah = ah + a[a.len() - i] * t;
        bh = bh * big_int + b[i - 1];
        if ah == bh {
            ans = i;
        }
        t *= big_int;
    }
    ans
}

fn main() {
    // let a = "earn".chars().map(|x| x as u64).collect::<Vec<_>>();
    let a = "earn".chars().map(|x| x as u64).collect::<Vec<_>>();
    let b = "learning".chars().map(|x| x as u64).collect::<Vec<_>>();
    assert_eq!(contains(&a, &b), true);
    let a = "earm".chars().map(|x| x as u64).collect::<Vec<_>>();
    let b = "learning".chars().map(|x| x as u64).collect::<Vec<_>>();
    assert_eq!(contains(&a, &b), false);

    let a = "deep".chars().map(|x| x as u64).collect::<Vec<_>>();
    let b = "epsilon".chars().map(|x| x as u64).collect::<Vec<_>>();
    assert_eq!(overlap(&a, &b), 2);
}
