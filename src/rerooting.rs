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

#[derive(Clone, Copy, Debug)]
struct DP<'a> {
    child: usize,
    pattern: Modulo,
    fact: &'a Vec<Modulo>,
    fact_inv: &'a Vec<Modulo>,
}

impl<'a> std::ops::Add for DP<'a> {
    type Output = DP<'a>;
    fn add(self, other: DP) -> DP<'a> {
        DP {
            child: self.child + other.child,
            pattern: self.pattern * other.pattern
                * comb(
                    self.child + other.child,
                    self.child,
                    self.fact,
                    self.fact_inv,
                ),
            fact: self.fact,
            fact_inv: self.fact_inv,
        }
    }
}

impl<'a> DP<'a> {
    fn add_root(&self) -> DP<'a> {
        let mut res = *self;
        res.child += 1;
        res
    }
}

impl<'a> std::ops::Sub for DP<'a> {
    type Output = DP<'a>;
    fn sub(self, other: DP) -> DP<'a> {
        DP {
            child: self.child - other.child,
            pattern: self.pattern
                * other.pattern.inv()
                * comb(self.child, other.child, self.fact, self.fact_inv).inv(),
            fact: self.fact,
            fact_inv: self.fact_inv,
        }
    }
}

fn dfs<'a>(cur: usize, parent: usize, graph: &Vec<Vec<usize>>, dp: &mut Vec<DP<'a>>) {
    for &to in graph[cur].iter() {
        if to == parent {
            continue;
        }
        dfs(to, cur, graph, dp);
        dp[cur] = dp[cur] + dp[to].add_root();
    }
}

fn bfs<'a>(cur: usize, parent: usize, graph: &Vec<Vec<usize>>, dp: &mut Vec<DP<'a>>) {
    for &to in graph[cur].iter() {
        if to == parent {
            continue;
        }
        let d = dp[cur] - dp[to].add_root();
        dp[to] = dp[to] + d.add_root();
        bfs(to, cur, graph, dp);
    }
}

fn main() {
    let n = read::<usize>();
    let mut graph = vec![vec![]; n];
    for i in 0..n - 1 {
        let v = read_vec::<usize>();
        let (a, b) = (v[0] - 1, v[1] - 1);
        graph[a].push(b);
        graph[b].push(a);
    }
    let mut fact = vec![Modulo(0); n + 1];
    let mut fact_inv = vec![Modulo(0); n + 1];

    fact[0] = Modulo(1);
    for i in 1..n + 1 {
        fact[i] = fact[i - 1] * i as i64;
    }
    for i in 0..n + 1 {
        fact_inv[i] = fact[i].inv();
    }

    let mut dp = vec![
        DP {
            child: 0,
            pattern: Modulo(1),
            fact: &fact,
            fact_inv: &fact_inv,
        };
        n
    ];

    dfs(0, 0, &graph, &mut dp);
    bfs(0, 0, &graph, &mut dp);
    for i in 0..n {
        println!("{}", dp[i].pattern);
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

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Modulo(i64);
static mut MODULUS: i64 = 1000_000_000 + 7;
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

fn comb(n: usize, k: usize, fact: &Vec<Modulo>, fact_inv: &Vec<Modulo>) -> Modulo {
    assert!(n >= k);
    fact[n] * fact_inv[n - k] * fact_inv[k]
}
