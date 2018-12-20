use std::cmp::Ordering;

pub trait BinarySearch<T> {
    fn upper_bound(&self, &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
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

#[derive(Clone, Debug)]
pub struct BitSet {
    buf: Vec<bool>,
    size: usize,
}

impl BitSet {
    pub fn new(size: usize, value: u32) -> BitSet {
        let mut buf = vec![false; size];
        let mut value = value;
        for i in 0..size {
            if value % 2 == 1 {
                buf[i] = true;
            }
            value = value >> 1;
        }
        BitSet {
            buf: buf,
            size: size,
        }
    }

    pub fn as_integer(&self) -> u32 {
        let mut value = 0;
        for i in (0..self.size).rev() {
            value = value << 1;
            if self.buf[i] {
                value += 1;
            }
        }
        value
    }
}

impl std::ops::Index<usize> for BitSet {
    type Output = bool;
    fn index(&self, index: usize) -> &bool {
        &self.buf[index]
    }
}

fn main() {
    let n: usize = 4;
    let w = vec![2, 1, 3, 2];
    let v = vec![3, 2, 4, 2];
    let max_w = 5;

    let n2 = n / 2;
    let mut s: Vec<(u32, u32)> = Vec::new();

    for i in 0..1 << n2 {
        let bs = BitSet::new(n2, i);
        let mut tw = 0;
        let mut tv = 0;

        for j in 0..n2 {
            if bs[j] {
                tw += w[j];
                tv += v[j];
            }
        }
        s.push((tw, tv));
    }
    s.sort();

    let mut highest_value = s[0].1;
    for i in 1..s.len() {
        if s[i].1 < highest_value {
            s[i].1 = highest_value;
        } else {
            highest_value = v[i];
        }
    }

    let sw = s.iter().map(|x| x.0).collect::<Vec<_>>();
    let sv = s.iter().map(|x| x.1).collect::<Vec<_>>();

    let mut ans = 0;
    for i in 0..1 << (n - n2) {
        let bs = BitSet::new(n - n2, i);
        let mut tw = 0;
        let mut tv = 0;
        for j in 0..n - n2 {
            if bs[j] {
                tw += w[j];
                tv += v[j];
            }
        }
        let key = std::cmp::min(sw.upper_bound(&(max_w - tw)), n2 - 1);
        if key == 0 {
            continue;
        }
        ans = std::cmp::max(ans, sv[key] + tv);
    }
    println!("{:?}", ans);
}
