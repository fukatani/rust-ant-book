use std::cmp::Ordering;
use std::collections::BinaryHeap;
const INF:i64 = 100000_00000;

#[derive(PartialEq, Debug)]
struct MinInt {
    value:i64,
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

fn make_pair(x: i64, y:usize) -> (MinInt, usize) {
    (MinInt{value: x}, y)
}

#[derive(Debug, Clone)]
struct Edge {
    to: usize,
    cost: i64
}

fn solve(edges: &Vec<Vec<Edge>>, start_idx: usize, end_idx: usize,
         num_apexes: usize) -> i64 {
    let mut dist: Vec<i64> = vec![INF; num_apexes];
    let mut dist2: Vec<i64> = vec![INF; num_apexes];
    dist[start_idx] = 0;
    let mut que = BinaryHeap::new();
    que.push(make_pair(0, start_idx));

    while let Some((u, v)) = que.pop() {
        if dist[v] < u.value {
            continue
        }
        for e in &edges[v] {
            let mut d2 = u.value + e.cost;
            if dist[e.to] > d2 {
                std::mem::swap(&mut d2, &mut dist[e.to]);
                que.push(make_pair(dist[e.to], e.to));
            }
            if dist2[e.to] > d2 && dist[e.to] < d2 {
                dist2[e.to] = d2;
                que.push(make_pair(dist2[e.to], e.to));
            }
        }
    }
    dist2[end_idx]
}

fn main() {
    let mut edges:Vec<Vec<Edge>> = vec![Vec::new(); 5];
    edges[1].push(Edge{to: 2, cost: 100});
    edges[2].push(Edge{to: 1, cost: 100});
    edges[2].push(Edge{to: 3, cost: 250});
    edges[3].push(Edge{to: 2, cost: 250});
    edges[2].push(Edge{to: 4, cost: 200});
    edges[4].push(Edge{to: 2, cost: 200});
    edges[3].push(Edge{to: 4, cost: 100});
    edges[4].push(Edge{to: 3, cost: 100});
    solve(&edges, 1, 4, 5);
}
