use std::cmp::Ordering;

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

pub trait BinarySearch<T> {
    fn lower_bound(&self, &T) -> usize;
    fn upper_bound(&self, &T) -> usize;
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
    let n: usize = read();
    let mut a: Vec<u64> = read_vec();
    let mut b: Vec<u64> = read_vec();
    a.sort();
    b.sort();

    let mut ans = 0;
    for c in 0..28 {
        let c = 2u64.pow(c + 1);
        let half_c = c / 2;
        let mut counts = 0;
        for num_a in a.iter() {
            let lb = b.lower_bound(&(*num_a + half_c));
            let ub = b.lower_bound(&((*num_a + c - 1) % c));
            if ub > lb {
                counts += ub - lb;
            } else {
                counts += n - ub;
                counts += lb;
            }
        }
        println!("{:?}", (half_c, counts));
        if counts % 2 == 1 {
            ans += half_c;
        }
    }
    println!("{:?}", ans);
}
