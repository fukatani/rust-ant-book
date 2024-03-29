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
impl std::ops::Neg for Modulo {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Modulo::new(-self.0)
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
    fact[n] * fact_inv[n -k] * fact_inv[k]
}

fn mod_comb2(n: i64, k: i64, invs: &Vec<Modulo>) -> Modulo {
    assert!(n >= k);
    assert!(invs.len() < 3 || invs[2].inv().0 == 2); // caution: not fact_inv

    let mut ret = Modulo(1);
    for i in 0..k {
        ret *= n - i;
        ret *= invs[i as usize + 1];
    }
    ret
}


type mat = Vec<Vec<Modulo>>;

fn mat_zeros(n: usize, m: usize) -> mat {
    vec![vec![Modulo(0); m]; n]
}

fn matmul(a: &mat, b: &mat) -> mat {
    let mut c = mat_zeros(a.len(), b[0].len());
    for i in 0..a.len() {
        for k in 0..b.len() {
            for j in 0..b[0].len() {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}

fn matpow(a: &mat, n: u64) -> mat {
    let mut b = mat_zeros(a.len(), a.len());
    for i in 0..a.len() {
        b[i][i] = Modulo(1);
    }
    let mut n = n;
    let mut a: mat = a.clone();
    while n > 0 {
        if n & 1 == 1 {
            b = matmul(&a, &b);
        }
        a = matmul(&a, &a);
        n >>= 1;
    }
    b
}

fn matinv(a: &mat) -> mat {
    assert!(a.len() == 2);
    assert!(a[0].len() == 2);
    let coef = (a[0][0] * a[1][1] - a[0][1] * a[1][0]).inv();
    vec![
        vec![a[1][1] * coef, -a[0][1] * coef],
        vec![-a[1][0] * coef, a[0][0] * coef],
    ]
}

fn main() {
    assert_eq!(Modulo::new(2).pow(10), Modulo::new(1024));
    assert_eq!(Modulo::new(2).pow(100), Modulo::new(1340));
    assert_eq!(Modulo::new(10000) + 10, Modulo::new(3));
    assert_eq!(Modulo::new(10000) * 2, Modulo::new(9993));
}
