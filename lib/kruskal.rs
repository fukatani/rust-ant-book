// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_2_A
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

fn kruskal(edges: &Vec<(i64, usize, usize)>, num_apexes: usize) -> i64 {
    let mut edges = edges.clone();
    let mut res = 0;
    edges.sort();
    let mut unf = UnionFindTree::new(num_apexes);
    for &(cost, to, from) in &edges {
        if !unf.same(to, from) {
            unf.unite(to, from);
            res += cost;
        }
    }
    if unf.get_size(0) != num_apexes {
        -1
    } else {
        res
    }
}

use std::cmp::*;
use std::collections::*;

fn main() {
    let v = read_vec::<usize>();
    let (v, e) = (v[0], v[1]);

    let mut edges = vec![];
    for i in 0..e {
        let v = read_vec::<usize>();
        let (a, b, c) = (v[2] as i64, v[0], v[1]);
        edges.push((a, b, c));
    }
    let ans = kruskal(&edges, v);
    println!("{}", ans);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}
