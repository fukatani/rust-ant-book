use std::cmp::Ordering;
use std::collections::*;

#[derive(Debug)]
struct MergeSegTree {
    n: usize,
    a: Vec<i64>,
    dat: Vec<Vec<i64>>,
}

fn merge_sort(a: &Vec<i64>, b: &Vec<i64>) -> Vec<i64> {
    let mut ptr_a = 0;
    let mut ptr_b = 0;
    let mut ret = Vec::with_capacity(a.len() + b.len());
    while ptr_a < a.len() || ptr_b < b.len() {
        if ptr_b == a.len() {
            ret.push(a[ptr_a]);
            ptr_a += 1;
        } else if ptr_a == a.len() || a[ptr_a] > b[ptr_b] {
            ret.push(b[ptr_b]);
            ptr_b += 1;
        } else {
            ret.push(a[ptr_a]);
            ptr_a += 1;
        }
    }
    ret
}

impl MergeSegTree {
    fn new(a: Vec<i64>) -> MergeSegTree {
        let mut m = 1;
        // For simplicity, we use 2 ** n sized SegTree.
        while m < a.len() {
            m *= 2;
        }
        let mut tree = MergeSegTree {
            n: m,
            a: a,
            dat: vec![vec![]; 2 * m - 1],
        };
        tree.init(0, 0, m);
        tree
    }

    fn init(&mut self, k: usize, l: usize, r: usize) {
        if r - l == 1 {
            if l < self.a.len() {
                self.dat[k].push(self.a[l]);
            } else {
                self.dat[k].push(std::i64::MAX);
            }
        } else {
            let lch = k * 2 + 1;
            let rch = k * 2 + 2;
            self.init(lch, l, (l + r) / 2);
            self.init(rch, (l + r) / 2, r);
            self.dat[k] = merge_sort(&self.dat[lch], &self.dat[rch]);
        }
    }

    // counts number <= x in [l, r)
    fn query_inner(&self, i: usize, j: usize, x: i64, k: usize, l: usize, r: usize) -> usize {
        if j <= l || r <= i {
            return 0;
        } else if i <= l && r <= j {
            return self.dat[k].upper_bound(&x);
        } else {
            let lc = self.query_inner(i, j, x, k * 2 + 1, l, (l + r) / 2);
            let rc = self.query_inner(i, j, x, k * 2 + 2, (l + r) / 2, r);
            return lc + rc;
        }
    }

    fn query(&self, i: usize, j: usize, x: i64) -> usize {
        self.query_inner(i, j, x, 0, 0, self.n)
    }
}

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}

fn main() {
    let a = vec![1, 5, 2, 6, 3, 7, 4];
    let queries = vec![(2, 5, 3), (4, 4, 1), (1, 7, 3)];
    let mut nums = a.clone();
    nums.sort();
    let segtree = MergeSegTree::new(a);
    println!("{:?}", segtree);
    for (l, r, k) in queries {
        let l = l - 1;
        let mut lb = 0;
        let mut ub = nums.len() - 1;
        while ub - lb > 1 {
            let md = (ub + lb) / 2;
            let c = segtree.query(l, r, nums[md]);
            if c < k {
                lb = md;
            } else {
                ub = md;
            }
        }
        println!("{}", nums[ub]);
    }
}
