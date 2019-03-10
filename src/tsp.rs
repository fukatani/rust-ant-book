const INF: u64 = 1000;

fn main() {
    let n = 5;
    let mut d = vec![vec![0u64; 5]; 5];
    for i in 0..n {
        for j in 0..5 {
            d[i][j] = INF;
        }
    }
    d[0][1] = 3;
    d[0][3] = 4;
    d[1][2] = 5;
    d[2][0] = 4;
    d[2][3] = 5;
    d[3][4] = 3;
    d[4][0] = 7;
    d[4][1] = 6;

    let mut dp = vec![vec![INF; 5]; 1 << n];
    dp[(1 << n) - 1][0] = 0;

    for S in (0..(1 << n) - 1).rev() {
        for v in 0..n {
            for u in 0..n {
                if (S >> u) & 1 == 0 {
                    dp[S][v] = std::cmp::min(dp[S][v], dp[S | 1 << u][u] + d[v][u]);
                }
            }
        }
    }
    println!("{}", dp[0][0]);
}
