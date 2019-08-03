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

fn main() {
    let v = read_vec::<i64>();
    let (n, d) = (v[0] as usize, v[1]);
    let a = read_vec::<i64>();
    if n == 1 {
        if a[0] % d == 0 {
            println!("{}", 1);
        } else {
            println!("{}", 0);
        }
        return;
    }
    let monoid = |a: i64, b: i64| (a * b) % d;
    let mut seg = SegTree::new(n, 1, monoid);
    for i in 0..n {
        seg.update(i, a[i]);
    }
    let mut sum_a = vec![0; n + 1];
    sum_a[0] = 0;
    for i in 0..n {
        sum_a[i + 1] = (sum_a[i] + a[i]) % d;
    }

    let mut sum_map = HashMap::new();
    for i in 0..n + 1 {
        sum_map.entry(sum_a[i]).or_insert(vec![]).push(i);
    }

    let mut ans = 0;
    for i in 0..n {
        let mut first_idx = binary_search(i as i64 - 1, n as i64 + 1, |x| {
            seg.query(i, x as usize + 1) != 0
        });
        let key = sum_a[i];
        if sum_map.contains_key(&key) {
            let search = sum_map[&key].lower_bound(&((first_idx + 1) as usize));
            ans += sum_map[&key].len() - search;
        }
    }
    println!("{}", ans);
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

struct SegTree<T, F>
where
    F: Fn(T, T) -> T,
    T: std::clone::Clone + std::marker::Copy,
{
    n: usize,
    dat: Vec<T>,
    init: T,
    functor: F,
}

impl<T, F> SegTree<T, F>
where
    F: Fn(T, T) -> T,
    T: std::clone::Clone + std::marker::Copy,
{
    fn new(n: usize, init: T, f: F) -> SegTree<T, F> {
        let mut m = 1;
        // For simplicity, we use 2 ** n sized SegTree.
        while m < n {
            m *= 2;
        }
        SegTree {
            n: m,
            dat: vec![init; 2 * m - 1],
            init: init,
            functor: f,
        }
    }

    // dat[k] = a;
    fn update(&mut self, k: usize, a: T) {
        let mut k = k;
        k += self.n - 1;
        self.dat[k] = a;
        while k > 0 {
            k = (k - 1) / 2;
            self.dat[k] = (self.functor)(self.dat[k * 2 + 1], self.dat[k * 2 + 2]);
        }
    }

    fn query(&self, a: usize, b: usize) -> T {
        self.query_inner(a, b, 0, 0, self.n)
    }

    fn query_inner(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        if r <= a || b <= l {
            return self.init;
        }
        if a <= l && r <= b {
            return self.dat[k];
        }

        let vl = self.query_inner(a, b, k * 2 + 1, l, (l + r) / 2);
        let vr = self.query_inner(a, b, k * 2 + 2, (l + r) / 2, r);
        (self.functor)(vl, vr)
    }
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

fn binary_search<F>(lb: i64, ub: i64, criterion: F) -> i64
where
    F: Fn(i64) -> bool,
{
    let mut ok = lb;
    let mut ng = ub;
    while ng - ok > 1 {
        let mid = (ng + ok) / 2;
        if criterion(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ng
}
