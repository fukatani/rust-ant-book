struct AdjacentMatrix {
    mat: Vec<Vec<usize>>,
}

impl AdjacentMatrix {
    pub fn new(n: usize) -> AdjacentMatrix {
        AdjacentMatrix{
            mat: vec![vec![0; n]; n],
        }
    }

    pub fn connect(&mut self, x: usize, y: usize) {
        self.mat[x][y] = 1;
        self.mat[y][x] = 1;
    }
}

struct Solver {
    adj_mat: AdjacentMatrix,
    colors: Vec<i32>
}

impl Solver {
    fn solve(&mut self, idx: usize, color: i32) -> bool {
        if self.colors[idx] != 0 && self.colors[idx] != color {
            return false;
        }
        self.colors[idx] = color;
        println!("{0} to {1}", idx, color);
        for i in 0..self.adj_mat.mat[idx].len() {
            if self.adj_mat.mat[idx][i] == 0 {
                continue;
            }
            if self.colors[i] == 0 {
                let result = self.solve(i, color * -1);
                if !result {
                    return false;
                }
            } else if self.colors[i] == color {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut adj_mat = AdjacentMatrix::new(4);
    adj_mat.connect(0, 1);
    adj_mat.connect(0, 3);
    adj_mat.connect(1, 2);
    adj_mat.connect(3, 2);
    let colors = vec![0; 4];
    let mut solver = Solver{adj_mat: adj_mat, colors: colors};
    println!("{:?}", solver.solve(0, 1));
}
