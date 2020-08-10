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
    let n = read::<usize>();

    let mut sz = 1;
    while sz < 2 * n + 1 {
        sz *= 2;
    }

    let mut f = vec![Complex::default(); sz];
    let mut g = vec![Complex::default(); sz];
    for i in 0..n {
        let v = read_vec::<f64>();
        f[i + 1] = Complex::new(v[0], 0.0);
        g[i + 1] = Complex::new(v[1], 0.0);
    }
    let c = fft_conv(&f, &g);
    for i in 1..2 * n + 1 {
        println!("{}", (c[i].re + 0.5) as i64);
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

#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    pub const fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }
}

impl Complex {
    /// Returns imaginary unit
    pub fn i() -> Self {
        Self::new(0.0, 1.0)
    }

    /// Calculate |self|
    pub fn norm(self) -> f64 {
        self.re.hypot(self.im)
    }
    pub fn arg(self) -> f64 {
        self.im.atan2(self.re)
    }
    pub fn to_polar(self) -> (f64, f64) {
        (self.norm(), self.arg())
    }
    pub fn from_polar(r: f64, theta: f64) -> Self {
        Self::new(r * theta.cos(), r * theta.sin())
    }
}

// (a + i b) + (c + i d) == (a + c) + i (b + d)
impl std::ops::Add<Complex> for Complex {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Self::Output::new(self.re + other.re, self.im + other.im)
    }
}

// (a + i b) - (c + i d) == (a - c) + i (b - d)
impl std::ops::Sub<Complex> for Complex {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Self::Output::new(self.re - other.re, self.im - other.im)
    }
}

// (a + i b) * (c + i d) == (a*c - b*d) + i (a*d + b*c)
impl std::ops::Mul<Complex> for Complex {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self::Output {
        let re = self.re.clone() * other.re.clone() - self.im.clone() * other.im.clone();
        let im = self.re * other.im + self.im * other.re;
        Self::Output::new(re, im)
    }
}

use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
impl AddAssign for Complex {
    fn add_assign(&mut self, other: Self) {
        self.re += other.re;
        self.im += other.im;
    }
}

impl SubAssign for Complex {
    fn sub_assign(&mut self, other: Self) {
        self.re -= other.re;
        self.im -= other.im;
    }
}

// (a + i b) * (c + i d) == (a*c - b*d) + i (a*d + b*c)
impl MulAssign for Complex {
    fn mul_assign(&mut self, other: Self) {
        let a = self.re.clone();

        self.re *= other.re.clone();
        self.re -= self.im.clone() * other.im.clone();

        self.im *= other.re;
        self.im += a * other.im;
    }
}
pub fn fft(xs: &mut Vec<Complex>, inv: bool) {
    let n = xs.len();
    assert!(n.count_ones() == 1);
    let h = n.trailing_zeros();
    for i in 0..n {
        let mut j = 0;
        for k in 0..h {
            j |= ((i >> k) & 1) << (h - 1 - k);
        }
        if i < j {
            xs.swap(i, j);
        }
    }

    for b in (0..h).map(|x| 1 << x) {
        for j in 0..b {
            let theta = if inv {
                2.0 * std::f64::consts::PI / (2.0 * b as f64) * j as f64
            } else {
                -2.0 * std::f64::consts::PI / (2.0 * b as f64) * j as f64
            };
            let w = Complex::from_polar(1.0, theta);
            for k in (0..n).step_by(2 * b) {
                let s = xs[j + k];
                let t = xs[j + k + b] * w;
                xs[j + k] = s + t;
                xs[j + k + b] = s - t;
            }
        }
    }

    if inv {
        for x in xs.iter_mut() {
            x.re /= n as f64;
            x.im /= n as f64;
        }
    }
}

fn fft_conv(xs: &Vec<Complex>, ys: &Vec<Complex>) -> Vec<Complex> {
    assert_eq!(xs.len(), ys.len());
    let mut xs = xs.clone();
    let mut ys = ys.clone();
    fft(&mut xs, false);
    fft(&mut ys, false);
    for i in 0..xs.len() {
        xs[i] *= ys[i];
    }
    fft(&mut xs, true);
    xs
}
