use std::cmp::Ordering;

#[derive(Debug, Clone)]
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

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}

struct UnionFindTree {
    parent_or_size: Vec<isize>,
}

impl UnionFindTree {
    fn new(size: usize) -> UnionFindTree {
        UnionFindTree { parent_or_size: vec![-1; size] }
    }

    fn find(&self, index: usize) -> usize {
        let mut index = index;
        while self.parent_or_size[index] >= 0 {
            index = self.parent_or_size[index] as usize;
        }
        index
    }

    fn same(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn unite(&mut self, index0: usize, index1: usize) -> bool {
        let a = self.find(index0);
        let b = self.find(index1);
        if a == b {
            false
        } else {
            if self.parent_or_size[a] < self.parent_or_size[b] {
                self.parent_or_size[a] += self.parent_or_size[b];
                self.parent_or_size[b] = a as isize;
            } else {
                self.parent_or_size[b] += self.parent_or_size[a];
                self.parent_or_size[a] = b as isize;
            }
            true
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
