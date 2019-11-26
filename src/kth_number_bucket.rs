use std::cmp::*;
use std::collections::*;

fn main() {
    let b = 1000;
    let a = vec![1, 5, 2, 6, 3, 7, 4];

    let n = a.len();
    let num_buckets = 100;
    let mut bucket = vec![vec![]; num_buckets];
    for i in 0..n {
        bucket[i / b].push(a[i]);
    }
    let mut nums = a.clone();
    nums.sort();

    for i in 0..num_buckets {
        bucket[i].sort();
    }

    let query = vec![(2, 5, 3), (4, 4, 1), (1, 7, 3)];

    let criterion = |mid: usize, l: usize, r: usize, k: usize| {
        let x = nums[mid];
        let mut tl = l;
        let mut tr = r;
        let mut c = 0;

        while tl < tr && tl % b != 0 {
            tr -= 1;
            if a[tr] <= x {
                c += 1;
            }
        }
        while tl < tr && tr % b != 0 {
            if a[tl] <= x {
                c += 1;
            }
            tl += 1;
        }

        // whole buckets
        while tl < tr {
            let bidx = tl / b;
            c += bucket[bidx].upper_bound(&x);
            tl += b;
        }

        c < k
    };

    for (l, r, k) in query {
        let l = l - 1;
        let (ok, ng) = binary_search(l, r, k, 0, n as i64 - 1, criterion);
        println!("{}", nums[ng as usize]);
    }
}

fn binary_search<F>(l: usize, r: usize, k: usize, lb: i64, ub: i64, criterion: F) -> (i64, i64)
where
    F: Fn(usize, usize, usize, usize) -> bool,
{
    let mut ok = lb;
    let mut ng = ub;
    while ng - ok > 1 {
        let mid = (ng + ok) / 2;
        if criterion(mid as usize, l, r, k) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    (ok, ng)
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
