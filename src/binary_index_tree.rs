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
    let mut bt = BinaryIndexTree::new(8);
    bt.add(1, 5);
    bt.add(2, 6);
    bt.add(3, 1);
    bt.add(6, -3);
    assert_eq!(bt.sum(1), 5);
    assert_eq!(bt.sum(2), 11);
    assert_eq!(bt.sum(4), 12);
    assert_eq!(bt.sum(7), 9);
}
