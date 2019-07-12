struct Solver {
    edges: Vec<Vec<usize>>,
    colors: Vec<i32>, // 0 uncolored, 1 or -1
}

impl Solver {
    fn new(n: usize) -> Solver {
        Solver {
            edges: vec![vec![]; n],
            colors: vec![0; n],
        }
    }
    fn solve(&mut self, idx: usize, color: i32) -> bool {
        if self.colors[idx] != 0 {
            return self.colors[idx] == color;
        }
        self.colors[idx] = color;
        for i in 0..self.edges[idx].len() {
            let to = self.edges[idx][i];
            if self.colors[to] == 0 {
                let result = self.solve(to, color * -1);
                if !result {
                    return false;
                }
            } else if self.colors[to] == color {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut solver = Solver::new(4);
    solver.edges[0].push(1);
    solver.edges[1].push(0);
    solver.edges[0].push(3);
    solver.edges[3].push(0);
    solver.edges[1].push(2);
    solver.edges[2].push(1);
    solver.edges[3].push(2);
    solver.edges[2].push(3);
    let colors = vec![0; 4];

    println!("{:?}", solver.solve(0, 1));
}
