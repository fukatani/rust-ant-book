#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($e:expr),*) => {
        #[cfg(debug_assertions)]
        $({
            let (e, mut err) = (stringify!($e), std::io::stderr());
            writeln!(err, "{} = {:?}", e, $e).unwrap()
        })*
    };
}

fn main() {
    let n = read::<usize>();
    let a = read_vec::<usize>()
        .iter()
        .map(|&x| x - 1)
        .collect::<Vec<_>>();
    let q = read::<usize>();
    let mut queries = vec![];
    for i in 0..q {
        let v = read_vec::<usize>();
        let (l, r) = (v[0] - 1, v[1] - 1);
        queries.push((l, r));
    }

    // let bucket = min((n as f64).sqrt().ceil() as usize, n);
    let bucket = 200;
    let mut q2 = vec![];
    for (i, (l, r)) in queries.into_iter().enumerate() {
        q2.push((l / bucket, r, l, i));
    }
    q2.sort();

    let mut answers = vec![0; q];
    let mut prev_lbi = std::usize::MAX;
    let mut count = vec![0; n];
    let mut temp = 0;
    let mut pointer = 0;
    let mut count_buf = vec![0; n];

    for (lbi, r, l, i) in q2 {
        let bound = (lbi + 1) * bucket;
        if lbi != prev_lbi {
            pointer = bound;
            for i in 0..n {
                count[i] = 0;
            }
            temp = 0;
            prev_lbi = lbi;
        }
        while pointer <= r {
            if count[a[pointer]] == 0 {
                count[a[pointer]] = 1;
            } else {
                count[a[pointer]] = 0;
                temp += 1;
            }
            pointer += 1;
        }
        answers[i] = temp;
        for lp in l..min(bound, r + 1) {
            count_buf[a[lp]] += 1;
        }

        for lp in l..min(bound, r + 1) {
            if count_buf[a[lp]] > 0 {
                answers[i] += (count_buf[a[lp]] + count[a[lp]]) / 2;
                count_buf[a[lp]] = 0;
            }
        }
    }

    for ans in answers {
        println!("{}", ans);
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}
