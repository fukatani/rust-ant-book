#[derive(Debug)]
struct SegTree {
    n: usize,
    dat: Vec<u32>
}

impl SegTree {
    fn new(n: usize) -> SegTree {
        let mut m = 1;
        // For simplicity, we use 2 ** n sized SegTree.
        while m < n {
            m *= 2;
        }
        SegTree {
            n: m,
            dat: vec![std::u32::MAX; 2 * m - 1]
        }
    }

    // dat[k] = a;
    fn update(&mut self, k: usize, a: u32) {
        let mut k = k;
        k += self.n - 1;
        self.dat[k] = a;
        while k > 0 {
            k = (k - 1) / 2;
            self.dat[k] = std::cmp::min(self.dat[k * 2 + 1], self.dat[k * 2 + 2]);
        }
    }

    fn query(&self, a: usize, b: usize) -> u32 {
        self.query_inner(a, b, 0, 0, self.n)
    }

    fn query_inner(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> u32 {
        if r <= a || b <= l {
            return std::u32::MAX;
        }
        if a <= l && r <= b {
            return self.dat[k];
        }

        let vl = self.query_inner(a, b, k * 2 + 1, l, (l + r) / 2);
        let vr = self.query_inner(a, b, k * 2 + 2, (l + r) / 2, r);
        std::cmp::min(vl, vr)
    }
}

fn main() {
    let mut seg = SegTree::new(6);
    seg.update(0, 4);
    seg.update(1, 2);
    seg.update(2, 10);
    seg.update(3, 9);
    seg.update(4, 8);
    seg.update(5, 1);

    println!("{:?}", seg.query(0, 3));
    println!("{:?}", seg.query(2, 5));
    println!("{:?}", seg.query(3, 6));
}
