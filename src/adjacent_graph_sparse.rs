struct AdjacentMatrix {
    mat: Vec<Vec<usize>>,
}

impl AdjacentMatrix {
    pub fn new(n: usize) -> AdjacentMatrix {
        AdjacentMatrix {
            mat: vec![vec![]; n],
        }
    }

    pub fn connect(&mut self, x: usize, y: usize) {
        self.mat[x].push(y);
        self.mat[y].push(x);
    }
}

struct Solver {
    adj_mat: AdjacentMatrix,
    colors: Vec<i32>,
}

impl Solver {
    fn solve(&mut self, idx: usize, color: i32) -> bool {
        if self.colors[idx] != 0 && self.colors[idx] != color {
            return false;
        }
        self.colors[idx] = color;
        let adjacents = self.adj_mat.mat[idx].iter().cloned().collect::<Vec<_>>();
        for i in adjacents {
            if self.colors[i] == 0 {
                let result = self.solve(i, color * -1);
                if !result {
                    return false;
                }
            } else if self.colors[i] == color {
                self.colors[i] = 2;
                return false;
            }
        }
        true
    }
}
