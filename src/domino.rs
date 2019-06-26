fn main() {
    let n = 3;
    let m = 3;
    let mut grid = vec![vec![false; m]; n];
    grid[1][1] = true;
    let mut dp = vec![0; 1 << m];
    dp[0] = 1;

    for i in (0..n).rev() {
        for j in (0..m).rev() {
            let mut next = vec![0; 1 << m];
            for used in 0..1 << m {
                if (used >> j) & 1 == 1 || grid[i][j] {
                    next[used] = dp[used & !(1 << j)];
                } else {
                    let mut res = 0;
                    if j + 1 < m && (used >> j + 1) & 1 == 0 && !grid[i][j + 1] {
                        res += dp[used | 1 << (j + 1)];
                    }
                    if i + 1 < n && !grid[i + 1][j] {
                        res += dp[used | 1 << j];
                    }
                    next[used] = res;
                }
            }
            dp = next;
        }
    }
    println!("{}", dp[0]);
}
