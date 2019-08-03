use std::collections::*;
use std::num::Wrapping;

fn compute_hash(a: &Vec<Vec<u64>>, n: usize, m: usize) -> Vec<Vec<Wrapping<u64>>> {
    let mut hash = vec![vec![Wrapping(0); m]; n];
    let a = a.iter()
        .map(|x| x.iter().map(|&y| Wrapping(y)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    const b1: Wrapping<u64> = Wrapping(9973u64);
    const b2: Wrapping<u64> = Wrapping(1000_000_000u64 + 7);
    let q = 2;

    let mut b1_q = Wrapping(1);
    for _ in 0..q {
        b1_q *= b1;
    }

    let mut tmp = vec![vec![Wrapping(0); m]; n];
    for i in 0..n {
        let mut e = Wrapping(0);
        for j in 0..q {
            e = e * b1 + a[i][j];
        }
        for j in 0..m + 1 - q {
            tmp[i][j] = e;
            if j + q < m {
                e = e * b1 - b1_q * a[i][j] + a[i][j + q];
            }
        }
    }

    let mut b2_q = Wrapping(1);
    for _ in 0..q {
        b2_q *= b2;
    }

    for j in 0..m + 1 - q {
        let mut e = Wrapping(0);
        for i in 0..q {
            e = e * b2 + tmp[i][j];
        }
        for i in 0..n + 1 - q {
            hash[i][j] = e;
            if i + q < m {
                e = e * b2 - b2_q * tmp[i][j] + tmp[i + q][j];
            }
        }
    }
    hash
}

fn main() {
    let n = 3;
    let m = 3;
    let q = 2;
    let pattern1 = vec!["**", "00"];
    let pattern2 = vec!["*0", "**"];
    let patterns = vec![pattern1, pattern2];
    let mut input_patterns = vec![];
    for p in patterns {
        input_patterns.push(
            p.iter()
                .map(|x| x.chars().map(|y| y as u64).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );
    }
    let field = vec!["*00", "0**", "*00"];
    let field = field
        .iter()
        .map(|x| x.chars().map(|y| y as u64).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut seen = HashMap::new();
    for pat in input_patterns.iter() {
        let hash = compute_hash(pat, q, q);
        seen.insert(hash[0][0], 0);
        println!("{}", hash[0][0]);
    }

    let hash = compute_hash(&field, n, m);
    for i in 0..n + 1 - q {
        for j in 0..m + 1 - q {
            if let Some(x) = seen.get_mut(&hash[i][j]) {
                *x += 1;
            }
        }
    }

    let mut ans = 0;
    for &c in seen.values() {
        if c != 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
