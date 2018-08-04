struct Solver {
    dp: Vec<Vec<i32>>,
    a: Vec<i32>,
    k: usize,
    m: Vec<usize>
}

impl Solver {
    fn new(n: usize, k: usize, a: Vec<i32>, m: Vec<usize>) -> Solver {
        let dp = vec![vec![-1; n]; k + 1];
        Solver{dp: dp, k: k, a: a, m: m}
    }

    fn solve(&mut self) {
        for i in 0..self.a.len() {
            for j in 0..self.k {
                if self.dp[i][j] >= 0 {
                    self.dp[i][j] = self.m[i];
                }
            }
        }
    }
}

fn main() {
    let n = 3;
    let k = 17;
    let a = vec![3, 5, 8];
    let m = vec![3, 2, 2];

    let solver = Solver::new(n, k, a, m);
}
