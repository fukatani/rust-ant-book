#[derive(Clone)]
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
    let mut ans = 0;
    if info.x > max_n || info.y > max_n {
        return 1;
    }
    if info.info_type == 1 {
        if uft.same(info.x, info.y + max_n) || uft.same(info.x, info.y + 2 * max_n) {
            ans += 1;
        } else {
            uft.unite(info.x, info.y);
            uft.unite(info.x + max_n, info.y + max_n);
            uft.unite(info.x + 2 * max_n, info.y + 2 * max_n);
        }
    } else {
        if uft.same(info.x, info.y) || uft.same(info.x, info.y + 2 * max_n) {
            ans += 1;
        } else {
            uft.unite(info.x, info.y + max_n);
            uft.unite(info.x + max_n, info.y + 2 * max_n);
            uft.unite(info.x + 2 * max_n, info.y);
        }
    }
    return ans;
}


fn main() {
    let max_n: usize = 100;
    let uft = UnionFindTree::new(3 * max_n);
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
        cnt += process_info(info, max_n, uft.clone());
    }
    println!("{:?}", cnt);
}
