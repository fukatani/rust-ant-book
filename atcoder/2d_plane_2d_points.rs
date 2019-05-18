#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::cmp::*;
use std::collections::*;
use std::io::Write;

#[allow(unused_macros)]
macro_rules! debug {
    ($($e:expr),*) => {
        #[cfg(debug_assertions)]
        $({
            let (e, mut err) = (stringify!($e), std::io::stderr());
            writeln!(err, "{} = {:?}", e, $e).unwrap()
        })*
    };
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

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    cap: i64,
    rev: usize,
}

#[derive(Clone)]
struct Solver {
    edges: Vec<Vec<Edge>>,
    used: Vec<bool>,
}

impl Solver {
    fn new(v: usize) -> Solver {
        Solver {
            edges: vec![Vec::new(); v],
            used: vec![false; v],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
        let rev = self.edges[to].len();
        self.edges[from].push(Edge {
            to: to,
            cap: cap,
            rev: rev,
        });
        let rev = self.edges[from].len() - 1;
        self.edges[to].push(Edge {
            to: from,
            cap: 0,
            rev: rev,
        });
    }

    fn dfs(&mut self, v: usize, t: usize, f: i64) -> i64 {
        if v == t {
            return f;
        }
        self.used[v] = true;
        for i in 0..self.edges[v].len() {
            let edge = self.edges[v][i];
            if !self.used[edge.to] && edge.cap > 0 {
                let d = self.dfs(edge.to, t, std::cmp::min(f, edge.cap));
                if d > 0 {
                    self.edges[v][i].cap -= d;
                    self.edges[edge.to][edge.rev].cap += d;
                    return d;
                }
            }
        }
        0
    }

    fn max_flow(&mut self, s: usize, t: usize) -> i64 {
        let mut flow = 0;
        loop {
            self.used = vec![false; self.edges.len()];
            let f = self.dfs(s, t, std::i64::MAX);
            if f == 0 {
                return flow;
            }
            flow += f;
        }
    }
}

fn biparate_matching(n: usize, k: usize, can: &Vec<Vec<bool>>) -> i64 {
    let s = n + k;
    let t = s + 1;
    let mut solver = Solver::new(n + k + 2);

    for i in 0..n {
        solver.add_edge(s, i, 1);
    }

    for i in 0..k {
        solver.add_edge(n + i, t, 1);
    }

    for i in 0..n {
        for j in 0..k {
            if can[i][j] {
                solver.add_edge(i, n + j, 1);
            }
        }
    }
    solver.max_flow(s, t)
}

fn main() {
    let n = read::<usize>();
    let mut red_points = Vec::new();
    for _ in 0..n {
        let v = read_vec::<i64>();
        red_points.push((v[0], v[1]));
    }
    let mut blue_points = Vec::new();
    for _ in 0..n {
        let v = read_vec::<i64>();
        blue_points.push((v[0], v[1]));
    }

    let mut can = vec![vec![false; n]; n];

    for (i, &(rx, ry)) in red_points.iter().enumerate() {
        for (j, &(bx, by)) in blue_points.iter().enumerate() {
            if rx < bx && ry < by {
                can[i][j] = true;
            }
        }
    }
    println!("{:?}", biparate_matching(n, n, &can));
}
