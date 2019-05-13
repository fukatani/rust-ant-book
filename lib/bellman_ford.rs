const INF:i32 = std::i32::MAX;

#[derive(Debug, Clone)]
struct Edge {
    from: usize,
    to: usize,
    cost: i32
}

#[derive(Debug)]
struct Solver {
    edges: Vec<Edge>,
    num_apexes: usize
}

impl Solver {
    fn new(edges: Vec<Edge>) -> Solver {
        let mut apex_set: std::collections::HashSet<usize> = std::collections::HashSet::new();
        for edge in &edges {
            apex_set.insert(edge.to);
            apex_set.insert(edge.from);
        }
        Solver{edges: edges, num_apexes: apex_set.len()}
    }

    fn solve(&self, start_idx: usize) {
        let mut cost = vec![0; self.num_apexes];
        cost[start_idx] = 0;
        loop {
            let mut updated = false;
            for e in &self.edges {
                if cost[e.to] > cost[e.from] + e.cost {
                    cost[e.to] = cost[e.from] + e.cost;
                    updated = true;
                }
            }
            if !updated {
                break;
            }
        }
        println!("{:?}", cost);
    }

    fn find_negative_loop(&self) -> bool {
        let mut d = vec![INF; self.num_apexes];
        for i in 0..self.num_apexes {
            for e in &self.edges {
                if d[e.from] != INF && d[e.to] > d[e.from] + e.cost {
                    d[e.to] = d[e.from] + e.cost;
                    if i == self.num_apexes - 1 {
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn main() {
    let mut edges:Vec<Edge> = Vec::new();
    edges.push(Edge{from: 0, to: 1, cost: 5});
    edges.push(Edge{from: 0, to: 2, cost: 4});
    edges.push(Edge{from: 1, to: 2, cost: -2});
    edges.push(Edge{from: 2, to: 3, cost: 2});
    edges.push(Edge{from: 2, to: 4, cost: 1});
    edges.push(Edge{from: 2, to: 5, cost: 4});
    edges.push(Edge{from: 4, to: 5, cost: 4});
    let solver = Solver::new(edges);
    // println!("{:?}", solver);
    solver.solve(0);
}
