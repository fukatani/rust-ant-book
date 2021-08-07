
use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct Edge {
    from: usize,
    to: usize,
    cost: i64,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Edge {}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}

#[derive(Debug, Clone)]
struct UnionFindTree {
    parent: Vec<isize>,
    size: Vec<usize>,
    height: Vec<u64>,
}

impl UnionFindTree {
    fn new(n: usize) -> UnionFindTree {
        UnionFindTree {
            parent: vec![-1; n],
            size: vec![1usize; n],
            height: vec![0u64; n],
        }
    }

    fn find(&mut self, index: usize) -> usize {
        if self.parent[index] == -1 {
            return index;
        }
        let idx = self.parent[index] as usize;
        let ret = self.find(idx);
        self.parent[index] = ret as isize;
        ret
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn get_size(&mut self, x: usize) -> usize {
        let idx = self.find(x);
        self.size[idx]
    }

    fn unite(&mut self, index0: usize, index1: usize) -> bool {
        let a = self.find(index0);
        let b = self.find(index1);
        if a == b {
            false
        } else {
            if self.height[a] > self.height[b] {
                self.parent[b] = a as isize;
                self.size[a] += self.size[b];
            } else if self.height[a] < self.height[b] {
                self.parent[a] = b as isize;
                self.size[b] += self.size[a];
            } else {
                self.parent[b] = a as isize;
                self.size[a] += self.size[b];
                self.height[a] += 1;
            }
            true
        }
    }
}

fn kruskal(edges: &Vec<Edge>, num_apexes: usize) -> i64 {
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
    if unf.get_size(0) != num_apexes {
        -1
    } else {
        res
    }
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
