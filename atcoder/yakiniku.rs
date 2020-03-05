fn main() {
    let v = read_vec::<usize>();
    let (n, k) = (v[0], v[1]);
    let mut meets = vec![];
    for _ in 0..n {
        let v = read_vec::<f64>();
        let (x, y, c) = (v[0], v[1], v[2]);
        meets.push((x, y, c));
    }

    let criterion = |t| -> bool {
        let mut circles = vec![];
        for &(x, y, c) in meets.iter() {
            circles.push(Circle {
                c: Point { x: x, y: y },
                r: t / c,
            });
        }
        let mut org_candidates = circles.iter().cloned().map(|x| x.c).collect::<Vec<_>>();

        for i in 0..n {
            for j in i + 1..n {
                let (cross1, cross2) = cross(circles[i], circles[j]);
                org_candidates.push(cross1);
                org_candidates.push(cross2);
            }
        }

        for org_cand in org_candidates {
            let count = meets
                .iter()
                .filter(|&&(x, y, c)| c * (x - org_cand.x).hypot(y - org_cand.y) <= t + 0.0000001)
                .count();
            if count >= k {
                return false;
            }
        }
        true
    };
    let (ok, ng) = binary_search(0.0, 200000.0, criterion);
    println!("{}", ng);
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

fn binary_search<F>(lb: f64, ub: f64, criterion: F) -> (f64, f64)
where
    F: Fn(f64) -> bool,
{
    let mut ok = lb;
    let mut ng = ub;
    while ng - ok > 0.0000002 {
        let mid = (ng + ok) / 2.;
        if criterion(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    (ok, ng)
}

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn norm(self) -> f64 {
        self.x.hypot(self.y)
    }

    fn arg(self) -> f64 {
        self.y.atan2(self.x)
    }
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Clone, Copy)]
struct Circle {
    c: Point,
    r: f64,
}

fn cross(c1: Circle, c2: Circle) -> (Point, Point) {
    let d = (c1.c - c2.c).norm();
    let a = ((c1.r * c1.r + d * d - c2.r * c2.r) / (2. * c1.r * d)).acos();
    let t = (c2.c - c1.c).arg();
    (c1.c + polar(c1.r, t + a), c1.c + polar(c1.r, t - a))
}

fn polar(a: f64, r: f64) -> Point {
    Point {
        x: r.cos() * a,
        y: r.sin() * a,
    }
}
