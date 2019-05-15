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
        self.edges[from].push(Edge { to, cap, rev });
        let rev = self.edges[from].len() - 1;
        self.edges[to].push(Edge {
            to: from,
            cap: 0,
            rev,
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

fn main() {
    let mut solver = Solver::new(10);
    let s = 0;
    let t = 4;

    solver.add_edge(s, 1, 10);
    solver.add_edge(s, 2, 2);
    solver.add_edge(1, 2, 6);
    solver.add_edge(1, 3, 6);
    solver.add_edge(3, 2, 3);
    solver.add_edge(2, t, 5);
    solver.add_edge(3, t, 8);

    assert_eq!(solver.max_flow(s, t), 11);
}
