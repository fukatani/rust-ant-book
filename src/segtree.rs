struct SegTree<T, F>
where
    F: Fn(T, T) -> T,
    T: std::clone::Clone + std::marker::Copy,
{
    n: usize,
    dat: Vec<T>,
    init: T,
    functor: F,
}

impl<T, F> SegTree<T, F>
where
    F: Fn(T, T) -> T,
    T: std::clone::Clone + std::marker::Copy,
{
    fn new(n: usize, init: T, f: F) -> SegTree<T, F> {
        let mut m = 1;
        // For simplicity, we use 2 ** n sized SegTree.
        while m < n {
            m *= 2;
        }
        SegTree {
            n: m,
            dat: vec![init; 2 * m - 1],
            init: init,
            functor: f,
        }
    }

    // dat[k] = a;
    fn update(&mut self, k: usize, a: T) {
        let mut k = k;
        k += self.n - 1;
        self.dat[k] = a;
        while k > 0 {
            k = (k - 1) / 2;
            self.dat[k] = (self.functor)(self.dat[k * 2 + 1], self.dat[k * 2 + 2]);
        }
    }

    fn query(&self, a: usize, b: usize) -> T {
        self.query_inner(a, b, 0, 0, self.n)
    }

    fn query_inner(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        if r <= a || b <= l {
            return self.init;
        }
        if a <= l && r <= b {
            return self.dat[k];
        }

        let vl = self.query_inner(a, b, k * 2 + 1, l, (l + r) / 2);
        let vr = self.query_inner(a, b, k * 2 + 2, (l + r) / 2, r);
        (self.functor)(vl, vr)
    }
}

fn main() {
    let mut seg = SegTree::new(6, std::u32::MAX, std::cmp::min);
    seg.update(0, 4);
    seg.update(1, 2);
    seg.update(2, 10);
    seg.update(3, 9);
    seg.update(4, 8);
    seg.update(5, 1);

    println!("{:?}", seg.query(0, 3));
    println!("{:?}", seg.query(2, 5));
    println!("{:?}", seg.query(3, 6));

    let mut seg = SegTree::new(6, 0u32, |x, y| x + y);
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
