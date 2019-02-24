struct BinaryIndexTree {
    bit: Vec<i64>,
    n: usize
}

impl BinaryIndexTree {
    fn new(n: usize) -> BinaryIndexTree {
        BinaryIndexTree{ bit: vec![0; n + 1], n: n}
    }

    fn sum(&self, i: usize) -> i64 {
        assert!(i > 0);
        let mut i = i;
        let mut s = 0i64;
        while  i > 0 {
            s += self.bit[i];
            i -= (i as i64 & -(i as i64)) as usize;
        }
        s
    }

    fn add(&mut self, i: usize, x: i64) {
        assert!(i > 0);
        let mut i = i;
        while i <= self.n {
            self.bit[i as usize] += x;
            i += (i as i64 & -(i as i64)) as usize;
        }
    }
}

fn main() {
    // let a: Vec<usize> = vec![1, 2];
    let a: Vec<usize> = vec![3, 2, 1];
    let mut bt = BinaryIndexTree::new(a.len());
    let mut ans = 0;
    for i in 0.. a.len() {
        ans += i - bt.sum(a[i]) as usize;
        println!("added {:?}", i - bt.sum(a[i]) as usize);
        bt.add(a[i] as usize, 1);
    }
    println!("{:?}", ans);
}
