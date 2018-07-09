const INF:i32 = std::i32::MAX;

#[derive(Debug)]
struct Solver {
    cost: Vec<Vec<i32>>,
    d: Vec<i32>,
    used: Vec<bool>,
    num_edges: usize
}

impl Solver {
    fn new(num_edges: usize) -> Solver {
        let d = vec![INF; num_edges];
        let used = vec![false; num_edges];
        let cost = vec![vec![INF; num_edges]; num_edges];
        Solver{cost: cost, d: d, used: used, num_edges: num_edges}
    }

    fn set_cost(&mut self, to: usize, from:usize, cost: i32) {
        self.cost[to][from] = cost;
        self.cost[from][to] = cost;
    }

    fn solve(&mut self, start: usize) {
        self.d[start] = 0;
        loop {
            let mut v = self.num_edges;
            for u in 0..self.num_edges {
                if !self.used[u] && (v == self.num_edges || self.d[u] < self.d[v]) {
                    v = u;
                }
            }
            if v == self.num_edges {
                break;
            }
            self.used[v] = true;
            for u in 0..self.num_edges {
                self.d[u] = std::cmp::min(self.d[u], self.d[v] + self.cost[u][v]);
            }
        }
        println!("{:?}", self.d);
    }
}

fn main() {
    let mut solver = Solver::new(6);
    solver.set_cost(0, 1, 5);
    solver.set_cost(0, 2, 4);
    solver.set_cost(1, 2, 2);
    solver.set_cost(2, 3, 2);
    solver.set_cost(2, 4, 1);
    solver.set_cost(2, 5, 4);
    solver.set_cost(4, 5, 4);
    solver.solve(0);
}
