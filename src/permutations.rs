struct Permutation {
    n: usize,
    idx: usize,
}
impl Permutation {
    fn new(n: usize) -> Permutation {
        Permutation { n: n, idx: 0 }
    }
}
impl Iterator for Permutation {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Vec<usize>> {
        let mut r = vec![0; self.n];
        let mut idx = self.idx;
        for k in 1..self.n {
            r[k] = idx % (k + 1);
            idx /= k + 1;
        }
        if idx > 0 {
            return None;
        }
        r.reverse();
        let mut b = vec![true; self.n];
        b[r[0]] = false;
        for k in 1..self.n {
            let mut count = 0;
            for j in 0..self.n {
                if b[j] {
                    if count == r[k] {
                        r[k] = j;
                        b[j] = false;
                        break;
                    }
                    count += 1;
                }
            }
        }
        self.idx += 1;
        return Some(r);
    }
}

fn main() {
    for ord in Permutation::new(3) {
        println!("{:?}", &ord);
    }
}
