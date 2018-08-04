fn solve(w: &Vec<usize>, v: &Vec<usize>, max_w: usize) -> usize {
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 10]; 10];
    for i in 0..w.len() {
        let i: usize = w.len() - 1 - i;
        for j in 0..max_w + 1 {
            if j < w[i] {
                dp[i][j] = dp[i + 1][j];
            } else {
                dp[i][j] = std::cmp::max(dp[i + 1][j], dp[i + 1][j - w[i]] + v[i]);
                println!("{0}, {1}, {2}", i, j, dp[i][j]);
            }
        }
    }
    return dp[0][max_w];
}

fn main() {
    let w: Vec<usize> = vec![2, 1, 3, 2];
    let v: Vec<usize> = vec![3, 2, 4, 2];

    let ans = solve(&w, &v, 5);
    println!("{:?}", ans);
}
