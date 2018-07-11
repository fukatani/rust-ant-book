fn main() {
    let p = 20;
    let q = 3;
    let a: Vec<usize> = vec![0, 3, 6, 14, 0];
    let mut dp:Vec<Vec<usize>> = vec![vec![0; q+1]; q+2];

    for w in 2..q+2 {
        for i in 0..q+2-w {
            let j = i + w;
            let mut t = std::usize::MAX;
            for k in i+1..j {
                t = std::cmp::min(t, dp[i][k] + dp[k][j]);
            }
            dp[i][j] = t + a[j] - a[i] - 2;
        }
    }
}
