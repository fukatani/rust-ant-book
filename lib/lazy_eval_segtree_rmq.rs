use std::cmp::*;

#[derive(Debug)]
struct LazySegmentTree {
    n: usize,
    node: Vec<i64>,
    lazy: Vec<i64>,
}

impl LazySegmentTree {
    fn new(sz: usize) -> LazySegmentTree {
        let mut n = 1;
        while n < sz {
            n *= 2;
        }
        LazySegmentTree {
            n: n,
            node: vec![std::i64::MAX; n * 2],
            lazy: vec![std::i64::MAX; n * 2],
        }
    }

    fn eval(&mut self, k: usize, l: usize, r: usize) {
        if self.lazy[k] != std::i64::MAX {
            self.node[k] = min(self.node[k], self.lazy[k]);
            if r - l > 1 {
                self.lazy[2 * k + 1] = min(self.lazy[2 * k + 1], self.lazy[k]);
                self.lazy[2 * k + 2] = min(self.lazy[2 * k + 2], self.lazy[k]);
            }
            self.lazy[k] = std::i64::MAX;
        }
    }

    fn update(&mut self, a: usize, b: usize, x: i64, k: usize, l: usize, r: usize) {
        // update [l, r) k = 0, l = 0, r = n is default
        self.eval(k, l, r);

        // if out of range, do nothing.
        if b <= l || r <= a {
            return;
        }

        if a <= l && r <= b {
            self.lazy[k] = min(self.lazy[k], x);
            self.eval(k, l, r);
        } else {
            self.update(a, b, x, 2 * k + 1, l, (l + r) / 2);
            self.update(a, b, x, 2 * k + 2, (l + r) / 2, r);
            self.node[k] = min(self.node[2 * k + 1], self.node[2 * k + 2]);
        }
    }

    fn getmin(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        if b <= l || r <= a {
            return std::i64::MAX;
        }

        self.eval(k, l, r);
        if a <= l && r <= b {
            return self.node[k];
        }

        min(self.getmin(a, b, 2 * k + 1, l, (l + r) / 2),
            self.getmin(a, b, 2 * k + 2, (l + r) / 2, r))
    }
}

fn main() {
    let n = 8usize;
    let mut st = LazySegmentTree::new(n);

    st.update(0, 8, 3, 0, 0, n);
    assert_eq!(3, st.getmin(0, 2, 0, 0, n));
    st.update(4, 7, 1, 0, 0, n);
    assert_eq!(1, st.getmin(3, 5, 0, 0, n));
    st.update(3, 6, -2, 0, 0, n);
    assert_eq!(-2, st.getmin(3, 5, 0, 0, n));
    assert_eq!(3, st.getmin(7, 8, 0, 0, n));
}
