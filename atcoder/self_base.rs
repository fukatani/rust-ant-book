#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::cmp::*;
use std::collections::*;
use std::io::Write;

#[allow(unused_macros)]
macro_rules! debug {
    ($($e:expr),*) => {
        #[cfg(debug_assertions)]
        $({
            let (e, mut err) = (stringify!($e), std::io::stderr());
            writeln!(err, "{} = {:?}", e, $e).unwrap()
        })*
    };
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Base {
    num: Vec<i64>,
}

impl std::cmp::Ord for Base {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.num.len() > other.num.len() {
            return Ordering::Greater;
        } else if self.num.len() < other.num.len() {
            return Ordering::Less;
        } else {
            for i in 0..self.num.len() {
                if self.num[i] > other.num[i] {
                    return Ordering::Greater;
                } else if self.num[i] < other.num[i] {
                    return Ordering::Less;
                }
            }
            return Ordering::Equal;
        }
    }
}

impl PartialOrd for Base {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn num_to_base(mut x: i64, a: i64) -> Base {
    let mut num = vec![];
    while x != 0 {
        num.push(x % a);
        x /= a;
    }
    num.reverse();
    Base { num: num }
}

fn base_to_num(num: Base, a: i64) -> i64 {
    let big_int = 1000_000_000 + 7;
    let mut ans = 0;
    let mut coef = 1;
    for i in (0..num.num.len()).rev() {
        ans += (num.num[i] * coef) % big_int;
        ans %= big_int;
        coef *= a;
        coef %= big_int;
    }
    ans
}

fn mul(num: &Base, coef: i64) -> Base {
    let mut num = num.num.clone();
    for _ in 0..coef {
        num.push(0);
    }
    Base { num: num }
}

fn main() {
    let v = read_vec::<i64>();
    let (n, A, mut B) = (v[0] as usize, v[1], v[2]);
    let mut a = read_vec::<i64>();
    if A == 1 {
        a.sort();
        for num in a {
            println!("{}", num);
        }
        return;
    }

    let mut nums = Vec::new();
    for i in 0..n {
        nums.push(num_to_base(a[i], A));
    }
    let mut a_min = nums.iter().cloned().min().unwrap();
    let mut a_max = nums.iter().cloned().max().unwrap();

    let mut nums = nums.into_iter().collect::<PriorityQueue<_>>();
    while mul(&a_min, 1) < a_max && B > 0 {
        let cur_min_mul = mul(&nums.pop().unwrap(), 1);
        a_max = max(a_max, cur_min_mul.clone());
        nums.push(cur_min_mul);
        a_min = nums.peek().unwrap();
        B -= 1;
    }

    let sisuu = B / n as i64;
    let big_int = 1000_000_000 + 7;
    let coef = pow_m(A, sisuu, big_int);
    B %= n as i64;

    for _ in 0..B {
        let cur_min_mul = mul(&nums.pop().unwrap(), 1);
        nums.push(cur_min_mul);
    }
    while let Some(num) = nums.pop() {
        let ans = (coef * base_to_num(num, A)) % big_int;
        println!("{}", ans);
    }
}

fn pow_m(n: i64, mut p: i64, m: i64) -> i64 {
    let mut r = n;
    let mut ret = 1;
    while p > 0 {
        if p % 2 == 0 {
            r = r * r % m;
            p /= 2;
        } else {
            ret = ret * r % m;
            p -= 1;
        }
    }
    ret
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

#[derive(Clone, Debug)]
struct PriorityQueue<T>
where
    T: std::cmp::Ord + std::clone::Clone,
{
    heap: Vec<T>,
}

impl<T> PriorityQueue<T>
where
    T: std::cmp::Ord + std::clone::Clone,
{
    fn new() -> PriorityQueue<T> {
        PriorityQueue {
            heap: Vec::<T>::new(),
        }
    }

    fn push(&mut self, x: T) {
        let mut i = self.heap.len() as i64;
        self.heap.push(x.clone());
        while i > 0 {
            let p = (i as usize - 1) / 2;
            if self.heap[p] <= x {
                break;
            }
            let temp = self.heap[i as usize].clone();
            self.heap[i as usize] = self.heap[p].clone();
            self.heap[p] = temp;
            i = p as i64;
        }
    }

    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let ret = self.heap[0].clone();
        let x = self.heap[self.heap.len() - 1].clone();
        let mut i = 0;
        let new_len = self.heap.len() - 1;
        while i * 2 + 1 < new_len {
            let mut a = i * 2 + 1;
            let b = i * 2 + 2;
            if b < new_len && self.heap[b] < self.heap[a] {
                a = b;
            }
            if self.heap[a] >= x {
                break;
            }
            self.heap[i] = self.heap[a].clone();
            i = a;
        }
        self.heap[i] = x;
        self.heap.swap_remove(new_len);
        Some(ret)
    }

    fn peek(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        Some(self.heap[0].clone())
    }
}

impl<T> std::iter::FromIterator<T> for PriorityQueue<T>
where
    T: std::cmp::Ord + std::clone::Clone,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut c = PriorityQueue::new();
        for i in iter {
            c.push(i);
        }
        c
    }
}
