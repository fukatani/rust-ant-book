use std::cmp::Ordering;
use std::collections::BinaryHeap;
const INF: i64 = 100000_00000_00000;

#[derive(PartialEq, Debug)]
struct MinInt {
    value: i64,
}

impl Eq for MinInt {}

impl PartialOrd for MinInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.value.partial_cmp(&self.value)
    }
}

impl Ord for MinInt {
    fn cmp(&self, other: &MinInt) -> Ordering {
        other.value.cmp(&self.value)
    }
}

fn make_pair(x: i64, y: usize) -> (MinInt, usize) {
    (MinInt { value: x }, y)
}

#[derive(Debug, Clone)]
struct Edge {
    to: usize,
    cost: i64,
}

fn solve(edges: &Vec<Vec<Edge>>, start_idx: usize) -> Vec<i64> {
    let num_apexes = edges.len();
    let mut d = vec![INF; num_apexes];
    d[start_idx] = 0;
    let mut que = BinaryHeap::new();
    que.push(make_pair(0, start_idx));

    while let Some((u, v)) = que.pop() {
        if d[v] < u.value {
            continue;
        }
        for e in &edges[v] {
            if d[v] != INF && d[e.to] > d[v] + e.cost {
                d[e.to] = d[v] + e.cost;
                que.push(make_pair(d[e.to], e.to));
            }
        }
    }
    d
}


fn main() {
    let mut edges:Vec<Vec<Edge>> = vec![Vec::new(); 6];
    edges[0].push(Edge{to: 1, cost: 5});
    edges[0].push(Edge{to: 2, cost: 4});
    edges[1].push(Edge{to: 2, cost: 2});
    edges[2].push(Edge{to: 3, cost: 2});
    edges[2].push(Edge{to: 4, cost: 1});
    edges[2].push(Edge{to: 5, cost: 4});
    edges[4].push(Edge{to: 5, cost: 4});
    solve(&edges, 0);
}
