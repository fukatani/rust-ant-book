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

fn get_options(
    start_idx: usize,
    end_idx: usize,
    w: i64,
    vs: &Vec<i64>,
    ws: &Vec<i64>,
) -> Vec<(i64, i64)> {
    let mut options_former = BTreeMap::new();
    for i in 0..1 << (end_idx - start_idx) {
        let mut i = i;
        let mut cur_w = 0;
        let mut cur_v = 0;
        for j in start_idx..end_idx {
            if i % 2 == 1 {
                cur_w += ws[j];
                cur_v += vs[j];
            }
            i /= 2;
        }
        if cur_w <= w {
            if options_former.contains_key(&cur_w) {
                *options_former.get_mut(&cur_w).unwrap() = max(options_former[&cur_w], cur_v);
            } else {
                options_former.insert(cur_w, cur_v);
            }
        }
    }
    let mut options_former_vec = vec![];
    for &key in options_former.keys() {
        if let Some(&(_, prev_v)) = options_former_vec.iter().last() {
            if prev_v >= options_former[&key] {
                continue;
            }
        }
        options_former_vec.push((key, options_former[&key]));
    }
    options_former_vec
}

fn solve_1(n: usize, w: i64, vs: &Vec<i64>, ws: &Vec<i64>) {
    let former_end_idx = n / 2;
    let former_options = get_options(0, former_end_idx, w, vs, ws);
    let latter_options = get_options(former_end_idx, n, w, vs, ws);

    let mut ans = 0i64;
    for (cur_w, cur_v) in former_options {
        let idx = latter_options.upper_bound(&(w - cur_w, std::i64::MAX));
        if idx == 0 {
            ans = max(ans, cur_v);
            continue;
        }
        ans = max(ans, latter_options[idx - 1].1 + cur_v);
    }
    println!("{}", ans);
}

fn solve_2(n: usize, w: i64, vs: &Vec<i64>, ws: &Vec<i64>) {
    let ws = ws.into_iter().map(|we| *we as usize).collect::<Vec<_>>();
    let mut dp = vec![0i64; ws.iter().sum::<usize>() + 1];
    for i in 0..n {
        for j in (0..dp.len()).rev() {
            if j + ws[i] >= dp.len() {
                continue;
            }
            dp[j + ws[i]] = max(dp[j + ws[i]], dp[j] + vs[i]);
        }
    }
    let mut ans = 0;
    for j in 0..dp.len() {
        if j as i64 > w {
            break;
        }
        ans = max(ans, dp[j]);
    }
    println!("{}", ans);
}

fn solve_3(n: usize, w: i64, vs: &Vec<i64>, ws: &Vec<i64>) {
    let vs = vs.into_iter().map(|ve| *ve as usize).collect::<Vec<_>>();
    let mut dp = vec![std::i64::MAX; vs.iter().sum::<usize>() + 1];
    dp[0] = 0;
    for i in 0..n {
        for j in (0..dp.len()).rev() {
            if j + vs[i] >= dp.len() || dp[j] == std::i64::MAX {
                continue;
            }
            dp[j + vs[i]] = min(dp[j + vs[i]], dp[j] + ws[i]);
        }
    }
    let mut ans = 0;
    for j in 0..dp.len() {
        if dp[j] > w {
            continue;
        }
        ans = max(ans, j);
    }
    println!("{}", ans);
}

fn main() {
    let v = read_vec::<i64>();
    let (n, w) = (v[0] as usize, v[1]);
    let mut vs = vec![0i64; n];
    let mut ws = vec![0i64; n];
    for i in 0..n {
        let v = read_vec::<i64>();
        vs[i] = v[0];
        ws[i] = v[1];
    }

    if vs.iter().max().unwrap() <= &1000 {
        solve_3(n, w, &vs, &ws);
    } else if ws.iter().max().unwrap() <= &1000 {
        solve_2(n, w, &vs, &ws);
    } else {
        solve_1(n, w, &vs, &ws);
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

pub trait BinarySearch<T> {
    fn lower_bound(&self, &T) -> usize;
    fn upper_bound(&self, &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}
