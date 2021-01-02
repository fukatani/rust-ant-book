type Cost = i64;
const INF: Cost = 100000_00000_00000;

fn solve(edges: &Vec<Vec<(usize, Cost)>>, start_idx: usize) -> Vec<Cost> {
    let num_apexes = edges.len();
    let mut d = vec![INF; num_apexes];
    d[start_idx] = 0;
    let mut que = std::collections::BinaryHeap::new();
    que.push((std::cmp::Reverse(0), start_idx));

    while let Some((u, v)) = que.pop() {
        if d[v] < u.0 {
            continue;
        }
        for e in &edges[v] {
            if d[v] != INF && d[e.0] > d[v] + e.1 {
                d[e.0] = d[v] + e.1;
                que.push((std::cmp::Reverse(d[e.0]), e.0));
            }
        }
    }
    d
}

fn main() {
    let mut edges = vec![Vec::new(); 6];
    edges[0].push((1, 5));
    edges[0].push((2, 4));
    edges[1].push((2, 2));
    edges[2].push((3, 2));
    edges[2].push((4, 1));
    edges[2].push((5, 4));
    edges[4].push((5, 4));
    solve(&edges, 0);
}
