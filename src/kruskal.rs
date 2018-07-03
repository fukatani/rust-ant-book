use std::cmp::Ordering;

#[derive(Debug, Clone, PartialOrd)]
struct Edge {
    from: usize,
    to: usize,
    cost: i32
}

impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Edge {}

impl Ord for Edge {
    fn cmp(&self, other:&Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

struct UnionFindTree {
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


fn kruskal(edges: &Vec<Edge>, num_apexes: usize) -> i32 {
    let mut edges = edges.clone();
    let mut res = 0;
    edges.sort();
    let mut unf = UnionFindTree::new(num_apexes);
    for e in edges {
        if !unf.same(e.to, e.from) {
            unf.unite(e.to, e.from);
            res += e.cost;
        }
    }
    res
}

fn main() {
    let mut edges:Vec<Edge> = Vec::new();
    edges.push(Edge{from: 0, to: 1, cost: 5});
    edges.push(Edge{from: 0, to: 2, cost: 4});
    edges.push(Edge{from: 1, to: 2, cost: 2});
    edges.push(Edge{from: 2, to: 3, cost: 2});
    edges.push(Edge{from: 2, to: 4, cost: 1});
    edges.push(Edge{from: 2, to: 5, cost: 4});
    edges.push(Edge{from: 4, to: 5, cost: 4});
    println!("{:?}", kruskal(&edges, 6));
}
