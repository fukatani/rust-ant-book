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

fn main() {
    let v: Vec<i64> = read_vec();
    let n = v[0] as usize;
    let k = v[1];
    let a = (0..n).map(|_| read::<i64>() - k).collect::<Vec<_>>();
    let mut sums = vec![-k; n + 1];
    for i in 1..n + 1 {
        sums[i] = sums[i - 1] + a[i - 1];
    }

    let indexes = argsort(&sums);
    let mut c = vec![0; n + 1];
    for i in 0..n + 1 {
        c[indexes[i]] = n + 1 - i;
    }

    let mut bt = BinaryIndexTree::new(indexes.len());
    let mut ans = 0;
    for i in 0..indexes.len() {
        ans += i - bt.sum(c[i]) as usize;
        bt.add(c[i], 1);
    }
    println!("{:?}", ans);
}

fn argsort<T: std::cmp::Ord + std::marker::Copy>(v: &Vec<T>) -> Vec<usize> {
    let mut sort_v = Vec::new();
    for i in 0..v.len() {
        sort_v.push((v[i], i));
    }
    sort_v.sort();
    sort_v.iter().map(|x| x.1).collect::<Vec<usize>>()
}

struct BinaryIndexTree {
    bit: Vec<i64>,
    n: usize,
}

impl BinaryIndexTree {
    fn new(n: usize) -> BinaryIndexTree {
        BinaryIndexTree {
            bit: vec![0; n + 1],
            n: n,
        }
    }

    fn sum(&self, i: usize) -> i64 {
        let mut i = i;
        let mut s = 0i64;
        while i > 0 {
            s += self.bit[i];
            i -= (i as i64 & -(i as i64)) as usize;
        }
        s
    }

    fn add(&mut self, i: usize, x: i64) {
        let mut i = i;
        while i <= self.n {
            self.bit[i as usize] += x;
            i += (i as i64 & -(i as i64)) as usize;
        }
    }
}

