type Cost = i64;
const INF: Cost = 100000_00000_00000;

fn solve(edges: &Vec<Vec<(usize, Cost)>>, start_idx: usize) -> (Vec<Cost>, Vec<Option<usize>>) {
    let num_apexes = edges.len();
    let mut d = vec![INF; num_apexes];
    let mut from = vec![None; num_apexes];
    d[start_idx] = 0;
    let mut que = std::collections::BinaryHeap::new();
    que.push((std::cmp::Reverse(0), start_idx));

    while let Some((cur_cost, cur)) = que.pop() {
        if d[cur] < cur_cost.0 {
            continue;
        }
        for &(to, cost) in &edges[cur] {
            if d[cur] != INF && d[to] > d[cur] + cost {
                d[to] = d[cur] + cost;
                from[to] = Some(cur);
                que.push((std::cmp::Reverse(d[to]), to));
            }
        }
    }
    (d, from)
}

// https://judge.yosupo.jp/problem/shortest_path
fn main() {
    let v = read_vec::<usize>();
    let (n, m, s, t) = (v[0], v[1], v[2], v[3]);
    let mut edges = vec![vec![]; n];
    for i in 0..m {
        let v = read_vec::<i64>();
        let (a, b, c) = (v[0] as usize, v[1] as usize, v[2]);
        edges[a].push((b, c));
    }
    let (d, from) = solve(&edges, s);
    if d[t] == INF {
        println!("-1");
        return;
    }
    let mut path = vec![t];
    let mut cur = t;
    while cur != s {
        cur = from[cur].unwrap();
        path.push(cur);
    }
    path.reverse();
    println!("{} {}", d[t], path.len() - 1);
    for i in 1..path.len() {
        println!("{} {}", path[i - 1], path[i]);
    }
}

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

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
