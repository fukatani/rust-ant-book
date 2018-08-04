fn solve(a: &Vec<usize>, m: &Vec<usize>, k: usize) -> bool {
    let mut dp: Vec<Vec<i32>> = vec![vec![-1; k + 1]; a.len() + 1];
    for i in 0..a.len() {
        dp[i][0] = 0;
    }
    for i in 0..a.len() {
        for j in 0..k + 1 {
            if dp[i][j] >= 0 {
                dp[i + 1][j] = m[i] as i32;
            } else if j < a[i] || dp[i + 1][j - a[i]] <= 0 {
                dp[i + 1][j] = -1;
            } else {
                dp[i + 1][j] = dp[i + 1][j - a[i]] - 1;
            }
        }
    }
    println!("{:?}", dp);
    return dp[a.len()][k] >= 0;
}

fn main() {
    let a: Vec<usize> = vec![3, 5, 8];
    let m: Vec<usize> = vec![3, 2, 2];
    let k: usize = 17;

    if solve(&a, &m, k) {
        println!("Yes");
    } else {
        println!("No");
    }
}
