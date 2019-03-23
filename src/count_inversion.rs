fn main() {
    let v = vec![1, 4, 3, 2];
    println!("{}", count_inversion(&v));

    let v = vec![1, 2, 3, 4];
    println!("{}", count_inversion(&v));

    let v = vec![4, 3, 2, 1];
    println!("{}", count_inversion(&v));
}

fn count_inversion(v: &Vec<usize>) -> i64 {
    let mut ans = 0;
    let mut bt = BinaryIndexTree::new(v.len());
    for i in 0..v.len() {
        ans += i as i64 - bt.sum(v[i]);
        bt.add(v[i], 1);
    }
    ans
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

