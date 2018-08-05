fn solve(a: &Vec<usize>, n: usize, m: usize) -> usize {
    let mut dp: Vec<Vec<usize>> = vec![vec![0; m + 1]; n + 1];
    for i in 0..n + 1 {
        dp[i][0] = 1;
    }

    for i in 0..n {
        for j in 1..m + 1 {
            if j >= a[i] + 1 {
                dp[i + 1][j] = dp[i + 1][j - 1] + dp[i][j] - dp[i][j - 1 - a[i]];
            } else {
                dp[i + 1][j] = dp[i + 1][j - 1] + dp[i][j];
            }
        }
    }
    println!("{:?}", dp);
    return dp[n][m];
}

fn main() {
    let a: Vec<usize> = vec![1, 2, 3];
    let n: usize = 3;
    let m: usize = 3;
    println!("{:?}", solve(&a, n, m));
}
