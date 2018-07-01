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

struct Info {
    info_type: i32,
    x: usize,
    y: usize
}

fn process_info(info: Info, max_n: usize, mut uft: UnionFindTree) -> i32 {
    if info.x > max_n || info.y > max_n {
        return 1;
    }
    if info.info_type == 1 {
        uft.
    } else {

    }
    return 0;
}


fn main() {
    let N: usize = 100;
    let mut uft = UnionFindTree::new(3 * N);
    let info1 = Info{info_type: 1, x: 101, y: 1};
    let info2 = Info{info_type: 2, x: 1, y: 2};
    let info3 = Info{info_type: 2, x: 2, y: 3};
    let info4 = Info{info_type: 2, x: 3, y: 3};
    let info5 = Info{info_type: 1, x: 1, y: 3};
    let info6 = Info{info_type: 1, x: 3, y: 1};
    let info7 = Info{info_type: 1, x: 5, y: 5};
    let mut infos: Vec<Info> = Vec::new();
    infos.push(info1);
    infos.push(info2);
    infos.push(info3);
    infos.push(info4);
    infos.push(info5);
    infos.push(info6);
    infos.push(info7);

    let mut cnt = 0;
    for info in infos {
        cnt += process_info(info, uft);
    }
}
