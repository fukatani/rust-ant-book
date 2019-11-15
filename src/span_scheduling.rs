#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::cmp::*;
use std::collections::*;
use std::io::Write;

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

use std::cmp::*;

fn main() {
    let n = 3;
    let max_log_n = 3;
    let m = 10;
    let mut s = vec![0, 2, 6];
    let mut t = vec![5, 7, 9];

    for i in 0..n {
        if t[i] < s[i] {
            t[i] += m;
        }
    }
    for i in 0..n {
        let prev = s[i];
        s.push(prev + m);
        let prev = t[i];
        t.push(prev + m);
    }
    let mut ps = vec![];

    for i in 0..n {
        ps.push((t[i], i));
    }
    for i in 0..n {
        ps.push((s[i], n * 2 + i));
    }
    ps.sort();

    let mut next = vec![vec![0; n * 2]; max_log_n + 1];
    let mut last: i32 = -1;

    // calc next[0]
    for &(_, idx) in ps.iter().rev() {
        if idx < n * 2 {
            // t
            next[0][idx] = last;
        } else {
            // s
            let idx = idx - 2 * n;
            if last == -1 || t[last as usize] > t[idx] {
                last = idx as i32;
            }
        }
    }

    for k in 0..max_log_n {
        for i in 0..2 * n {
            if next[k][i] == -1 {
                next[k + 1][i] = -1;
            } else {
                next[k + 1][i] = next[k][next[k][i] as usize];
            }
        }
    }

    debug!(next);
    let mut ans = 0;
    for i in 0..n {
        let mut tmp = 0;
        let mut j = i;
        for k in (0..max_log_n).rev() {
            let mut j2 = next[k][j];
            if j2 >= 0 && t[j2 as usize] <= s[i] + m {
                j = j2 as usize;
                tmp |= 1 << k;
            }
        }
        ans = max(ans, tmp + 1);
    }
    println!("{}", ans);
}
