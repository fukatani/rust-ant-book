#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::cmp::*;
use std::collections::*;

use std::fmt::{Debug, Display, Formatter, Write as FmtWrite};
use std::io::{stderr, stdin, BufRead, Write};

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
    let v = read_vec::<usize>();
    let (h, w, n) = (v[0] as i32, v[1] as i32, v[2]);
    let v = read_vec::<i32>();
    let (start_y, start_x) = (v[0], v[1]);
    let s = read::<String>().chars().collect::<Vec<_>>();
    let t = read::<String>().chars().collect::<Vec<_>>();

    let mut safe = true;

    for &(start, limit, R, L) in &[(start_x, w, 'R', 'L'), (start_y, h, 'D', 'U')] {
        let mut l = 1;
        let mut r = limit;
        for i in (0..n).rev() {
            if i != n - 1 {
                if t[i] == R && l - 1 >= 1 {
                    l -= 1;
                }
                if t[i] == L && r + 1 <= limit {
                    r += 1;
                }
                debug!((i, "aoki", l, r, safe));
            }

            if s[i] == R {
                r -= 1;
            }
            if s[i] == L {
                l += 1;
            }

            safe &= l >= 1 && l <= r && r <= limit;
            debug!((i, "taka", l, r, safe));
        }
        if start < l || start > r {
            safe = false;
        }
    }
    if safe {
        println!("YES");
    } else {
        println!("NO");
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
