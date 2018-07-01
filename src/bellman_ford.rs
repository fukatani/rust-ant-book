const INF:i32 = 1000;

#[derive(Debug)]
#[derive(Clone)]
struct Edge {
    from: usize,
    to: usize,
    cost: i32
}

#[derive(Debug)]
struct Solver {
    edges: Vec<Edge>,
    num_edges: usize,
    num_apexes: usize
}

impl Solver {
    fn new(edges: Vec<Edge>) -> Solver {
        let mut apex_set: std::collections::HashSet<usize> = std::collections::HashSet::new();
        let edge2 = edges.clone();
        for edge in edges {
            apex_set.insert(edge.to);
            apex_set.insert(edge.from);
        }
        Solver{num_edges: edge2.len(), edges: edge2, num_apexes: apex_set.len()}
    }

    fn solve(self, start_idx: usize) {
        let mut cost = vec![INF; self.num_apexes];
        cost[start_idx] = 0;
        loop {
            let mut updated = false;
            for e in &self.edges {
                if cost[e.from] != INF && cost[e.to] > cost[e.from] + e.cost {
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
