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

fn main() {
    assert_eq!(Modulo::new(2).pow(10), Modulo::new(1024));
    assert_eq!(Modulo::new(2).pow(100), Modulo::new(1340));
    assert_eq!(Modulo::new(10000) + 10, Modulo::new(3));
    assert_eq!(Modulo::new(10000) * 2, Modulo::new(9993));
}
