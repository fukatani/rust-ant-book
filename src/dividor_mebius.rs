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
    let a = read_vec::<usize>();
    let mut edges = vec![vec![]; n];
    for i in 0..n - 1 {
        let v = read_vec::<usize>();
        let (a, b) = (v[0] - 1, v[1] - 1);
        edges[a].push(b);
        edges[b].push(a);
    }

    let ps = get_primes(100000)
        .iter()
        .map(|&x| x as usize)
        .collect::<Vec<_>>();
    let mut ok = vec![vec![]; 100001];
    for i in 0..n {
        ok[a[i]].push(i);
    }

    for &p in &ps {
        let mut i = 100000 / p;
        while i >= 1 {
            let mut c = ok[i * p].clone();
            ok[i].append(&mut c);
            i -= 1;
        }
    }

    let mut f = vec![Modulo(0); 100001];
    let mut enable = vec![false; n];
    let mut visited = vec![false; n];
    let mut dfs1_result = vec![0; n];
    let two_inv = Modulo(2).inv();
    for x in 1..=100000 {
        for &i in &ok[x] {
            enable[i] = true;
        }

        for &i in &ok[x] {
            if visited[i] {
                continue;
            }
            dfs1_result[i] = dfs(i, &edges, &mut visited, &enable, &mut Modulo(0), 0);
        }
        for &i in &ok[x] {
            visited[i] = false;
        }

        for &i in &ok[x] {
            if visited[i] {
                continue;
            }
            let mut temp = Modulo(0);
            dfs(i, &edges, &mut visited, &enable, &mut temp, dfs1_result[i]);
            f[x] += temp;
            f[x] += two_inv * dfs1_result[i] * (dfs1_result[i] - 1);
        }

        for &i in &ok[x] {
            enable[i] = false;
            visited[i] = false;
        }
    }

    for &p in &ps {
        for i in 1.. {
            if i * p > 100000 {
                break;
            }
            let val = f[i * p];
            f[i] -= val;
        }
    }

    let ans = (1..=100000).map(|x| f[x] * x as i64).sum::<Modulo>();
    println!("{}", ans);
}

fn dfs(
    cur: usize,
    edges: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    enable: &Vec<bool>,
    path_count: &mut Modulo,
    total: i64,
) -> i64 {
    let mut ret = 1;
    visited[cur] = true;
    for &to in edges[cur].iter() {
        if !enable[to] || visited[to] {
            continue;
        }

        let temp = dfs(to, edges, visited, enable, path_count, total);
        ret += temp;
        *path_count += temp * (total - temp);
    }
    ret
}

fn get_primes(n: i64) -> Vec<i64> {
    let mut is_prime = vec![true; n as usize + 1];
    let mut primes = Vec::new();
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..n + 1 {
        if is_prime[i as usize] {
            primes.push(i);
            let mut j = 2 * i;
            while j <= n {
                is_prime[j as usize] = false;
                j += i;
            }
        }
    }
    primes
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

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Modulo(i64);
// static mut MODULUS: i64 = 1000_000_000 + 7;
static mut MODULUS: i64 = 998244353;
impl Modulo {
    fn set_modulus(m: i64) {
        unsafe {
            MODULUS = m;
        }
    }
    fn get_modulus() -> i64 {
        unsafe { MODULUS }
    }
    fn new(x: i64) -> Modulo {
        let m = Modulo::get_modulus();
        if x < 0 {
            Modulo(x % m + m)
        } else if x < m {
            Modulo(x)
        } else {
            Modulo(x % m)
        }
    }
    fn pow(self, p: i64) -> Modulo {
        if p == 0 {
            Modulo(1)
        } else {
            let mut t = self.pow(p / 2);
            t *= t;
            if p & 1 == 1 {
                t *= self;
            }
            t
        }
    }
    fn inv(self) -> Modulo {
        self.pow(Modulo::get_modulus() - 2)
    }
}
impl std::fmt::Display for Modulo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::ops::AddAssign for Modulo {
    fn add_assign(&mut self, other: Modulo) {
        let m = Modulo::get_modulus();
        self.0 += other.0;
        if self.0 >= m {
            self.0 -= m;
        }
    }
}
impl std::ops::MulAssign for Modulo {
    fn mul_assign(&mut self, other: Modulo) {
        let m = Modulo::get_modulus();
        self.0 *= other.0;
        self.0 %= m;
    }
}
impl std::ops::SubAssign for Modulo {
    fn sub_assign(&mut self, other: Modulo) {
        let m = Modulo::get_modulus();
        self.0 += m - other.0;
        if self.0 >= m {
            self.0 -= m;
        }
    }
}
macro_rules! impl_modulo_ops {
    ($imp:ident, $method:ident, $assign_imp:ident, $assign_method:ident) => {
        impl<'a> std::ops::$assign_imp<&'a Modulo> for Modulo {
            fn $assign_method(&mut self, other: &'a Modulo) {
                std::ops::$assign_imp::$assign_method(self, *other);
            }
        }
        impl std::ops::$imp for Modulo {
            type Output = Modulo;
            fn $method(self, other: Modulo) -> Modulo {
                let mut x = self;
                std::ops::$assign_imp::$assign_method(&mut x, other);
                x
            }
        }
        impl<'a> std::ops::$imp<Modulo> for &'a Modulo {
            type Output = Modulo;
            fn $method(self, other: Modulo) -> Modulo {
                std::ops::$imp::$method(*self, other)
            }
        }
        impl<'a> std::ops::$imp<&'a Modulo> for Modulo {
            type Output = Modulo;
            fn $method(self, other: &'a Modulo) -> Modulo {
                std::ops::$imp::$method(self, *other)
            }
        }
        impl<'a, 'b> std::ops::$imp<&'b Modulo> for &'a Modulo {
            type Output = Modulo;
            fn $method(self, other: &'b Modulo) -> Modulo {
                std::ops::$imp::$method(*self, *other)
            }
        }
        impl std::ops::$assign_imp<i64> for Modulo {
            fn $assign_method(&mut self, other: i64) {
                std::ops::$assign_imp::$assign_method(self, Modulo::new(other));
            }
        }
        impl<'a> std::ops::$assign_imp<&'a i64> for Modulo {
            fn $assign_method(&mut self, other: &'a i64) {
                std::ops::$assign_imp::$assign_method(self, *other);
            }
        }
        impl std::ops::$imp<i64> for Modulo {
            type Output = Modulo;
            fn $method(self, other: i64) -> Modulo {
                let mut x = self;
                std::ops::$assign_imp::$assign_method(&mut x, other);
                x
            }
        }
        impl<'a> std::ops::$imp<&'a i64> for Modulo {
            type Output = Modulo;
            fn $method(self, other: &'a i64) -> Modulo {
                std::ops::$imp::$method(self, *other)
            }
        }
        impl<'a> std::ops::$imp<i64> for &'a Modulo {
            type Output = Modulo;
            fn $method(self, other: i64) -> Modulo {
                std::ops::$imp::$method(*self, other)
            }
        }
        impl<'a, 'b> std::ops::$imp<&'b i64> for &'a Modulo {
            type Output = Modulo;
            fn $method(self, other: &'b i64) -> Modulo {
                std::ops::$imp::$method(*self, *other)
            }
        }
    };
}
impl_modulo_ops!(Add, add, AddAssign, add_assign);
impl_modulo_ops!(Mul, mul, MulAssign, mul_assign);
impl_modulo_ops!(Sub, sub, SubAssign, sub_assign);

use std::iter::Sum;
impl Sum for Modulo {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Modulo>,
    {
        iter.fold(Modulo(0), |a, b| a + b)
    }
}

impl<'a> Sum<&'a Modulo> for Modulo {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Modulo(0), |a, b| a + b)
    }
}

use std::iter::Product;
impl Product for Modulo {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Modulo>,
    {
        iter.fold(Modulo(1), |a, b| a * b)
    }
}

impl<'a> Product<&'a Modulo> for Modulo {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Modulo(1), |a, b| a * b)
    }
}

fn mod_comb(n: usize, k: usize, fact: &[Modulo], fact_inv: &[Modulo]) -> Modulo {
    assert!(n >= k);
    fact[n] * fact_inv[n - k] * fact_inv[k]
}
