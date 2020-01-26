use std::cmp::*;

#[derive(Debug)]
struct LazySegmentTree {
    n: usize,
    data: Vec<i64>,
    datb: Vec<i64>,
}

impl LazySegmentTree {
    fn new(sz: usize) -> LazySegmentTree {
        let mut n = 1;
        while n < sz {
            n *= 2;
        }
        LazySegmentTree {
            n: n,
            data: vec![0i64; 2 * n - 1],
            datb: vec![0i64; 2 * n - 1],
        }
    }

    fn add(&mut self, a: usize, b: usize, x: i64, k: usize, l: usize, r: usize) {
        if a <= l && r <= b {
            self.data[k] += x;
        } else if l < b && a < r {
            self.datb[k] += (min(b, r) - max(a, l)) as i64 * x;
            self.add(a, b, x, k * 2 + 1, l, (l + r) / 2);
            self.add(a, b, x, k * 2 + 2, (l + r) / 2, r);
        }
    }

    fn getsum(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        if b <= l || r <= a {
            return 0;
        }
        if a <= l && r <= b {
            return self.data[k] * (r - l) as i64 + self.datb[k];
        } else {
            let mut res = (min(b, r) - max(a, l)) as i64 * self.data[k];
            res += self.getsum(a, b, k * 2 + 1, l, (l + r) / 2);
            res += self.getsum(a, b, k * 2 + 2, (l + r) / 2, r);
            return res;
        }
    }
}

fn main() {
    let n = 8usize;
    let mut st = LazySegmentTree::new(n);
    // st.add(3, 6, 2, 0, 0, n);
    st.add(0, 8, -3, 0, 0, n);
    assert_eq!(-6, st.getsum(0, 2, 0, 0, n));
    st.add(4, 7, 1, 0, 0, n);
    assert_eq!(-5, st.getsum(3, 5, 0, 0, n));
    // println!("{:?}", st);
}
