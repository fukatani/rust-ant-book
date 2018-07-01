struct UnionFindTree {
    n: usize,
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFindTree {
    pub fn new(n: usize) -> UnionFindTree {
        let mut par: Vec<usize> = Vec::new();
        for i in 0..n {
            par.push(i);
        }
        let rank = vec![0; n];
        UnionFindTree{
            n: n,
            par: par,
            rank: rank,
        }
    }

    fn find(&self, x: usize) -> usize {
        let mut x = x;
        while self.par[x] != x {
            x = self.par[x];
        }
        x
    }

    fn same(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn unite(&mut self, x: usize, y: usize) {
        if self.same(x, y) {
            return;
        }
        let mut x = x;
        let mut y = y;
        if self.rank[y] > self.rank[x] {
            std::mem::swap(&mut x, &mut y);
        }
        self.par[x] = y;
        if self.rank[x] == self.rank[y] {
          self.rank[x] = self.rank[y] + 1;
      }
    }
}


fn main() {
    let mut uft = UnionFindTree::new(4);
    println!("{:?}", uft.par);
    println!("{:?}", uft.same(0, 1));
    uft.unite(0, 1);
    println!("{:?}", uft.same(0, 1));
    println!("{:?}", uft.same(0, 2));
    uft.unite(2, 1);
    println!("{:?}", uft.par);
    println!("{:?}", uft.same(0, 2));
}
