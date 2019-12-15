use std::cmp::*;
use std::collections::*;

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

fn add(a: f64, b: f64) -> f64 {
    let epsillon = 1e-10;
    if (a + b).abs() < epsillon * (a.abs() + b.abs()) {
        return 0.0;
    }
    a + b
}

#[derive(Clone, Copy)]
struct P {
    x: f64,
    y: f64,
}

impl std::ops::Add for P {
    type Output = P;
    fn add(self, other: P) -> P {
        P {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for P {
    type Output = P;
    fn sub(self, other: P) -> P {
        P {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl P {
    fn dot(self, other: P) -> f64 {
        add(self.x * other.x, self.y * other.y)
    }
    fn cross(self, other: P) -> f64 {
        add(self.x * other.y, -self.y * other.x)
    }
    fn mul(self, other: f64) -> P {
        P {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

fn on_seg(p1: P, p2: P, q: P) -> bool {
    (p1 - q).cross(p2 - q) == 0.0 && (p1 - q).dot(p2 - q) <= 0.0
}

fn intersection(p1: P, p2: P, q1: P, q2: P) -> P {
    p1 + (p2 - p1).mul((q2 - q1).cross(q1 - p1) / (q2 - q1).cross(p2 - p1))
}

fn main() {
    let n = 4;
    let p = vec![
        P { x: 0.0, y: 4.0 },
        P { x: 0.0, y: 1.0 },
        P { x: 1.0, y: 2.0 },
        P { x: 1.0, y: 0.0 },
    ];
    let q = vec![
        P { x: 4.0, y: 1.0 },
        P { x: 2.0, y: 3.0 },
        P { x: 3.0, y: 4.0 },
        P { x: 2.0, y: 1.0 },
    ];
    let m = 4;
    let pairs = vec![(1, 2), (1, 4), (2, 3), (2, 4)];

    let mut g = vec![vec![false; n]; n];
    for i in 0..n {
        g[i][i] = true;
        for j in 0..i {
            if (p[i] - q[i]).cross(p[j] - q[j]) == 0.0 {
                g[i][j] = on_seg(p[i], q[i], p[j])
                    || on_seg(p[i], q[i], q[j])
                    || on_seg(p[j], q[j], p[i])
                    || on_seg(p[j], q[j], q[i]);
            } else {
                let r = intersection(p[i], q[i], p[j], q[j]);
                g[i][j] = on_seg(p[i], q[i], r) && on_seg(p[j], q[j], r);
            }
            g[j][i] = g[i][j];
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                g[i][j] |= g[i][k] && g[k][j];
            }
        }
    }

    for (a, b) in pairs {
        if g[a - 1][b - 1] {
            println!("CONNECTED");
        } else {
            println!("NOT CONNECTED");
        }
    }
}
