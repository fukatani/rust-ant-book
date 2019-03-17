#[derive(Debug)]
struct LazySegmentTree {
    n: usize,
    node: Vec<i64>,
    lazy: Vec<i64>,
}

impl LazySegmentTree {
    fn new(n: usize) -> LazySegmentTree {
        LazySegmentTree {
            n: n,
            node: vec![0i64; n],
            lazy: vec![0i64; n],
        }
    }

    fn eval(&mut self, k: usize, l: usize, r: usize) {
        if self.lazy[k] != 0 {
            self.node[k] += self.lazy[k];
            if r - l > 1 {
                self.lazy[2 * k + 1] += self.lazy[k] / 2;
                self.lazy[2 * k + 2] += self.lazy[k] / 2;
            }
            self.lazy[k] = 0;
        }
    }

    fn add(&mut self, a: usize, b: usize, x: i64, k: usize, l: usize, r: usize) {
        self.eval(k, l, r);

        // if out of range, do nothing.
        if b <= l || r <= a {
            return;
        }

        if a <= l && r <= b {
            self.lazy[k] += (r as i64 - 1) * x;
            self.eval(k, l, r);
        } else {
            self.add(a, b, x, 2 * k + 1, l, (l + r) / 2);
            self.add(a, b, x, 2 * k + 2, (l + r) / 2, r);
            self.node[k] = self.node[2 * k + 1] + self.node[2 * k + 2];
        }
    }

    fn getsum(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        if b <= l || r <= a {
            return 0;
        }

        self.eval(k, l, r);
        if a <= l && r <= b {
            return self.node[k];
        }

        self.getsum(a, b, 2 * k + 1, l, (l + r) / 2)
            + self.getsum(a, b, 2 * k + 2, (l + r) / 2, r)
    }
}

fn main() {
    let n = 8usize;
    let mut st = LazySegmentTree::new(n);
    st.add(3, 6, 2, 0, n, n);
    st.add(4, 7, 1, 0, n, n);
    st.add(0, 8, -3, 0, n, n);
    println!("{:?}", st.getsum(0, 5, 0, 0, n));
    println!("{:?}", st);
}
