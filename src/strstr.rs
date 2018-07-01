
fn rec(s: &str, t: &str, dp: &mut Vec<Vec<usize>>) -> usize {
    for i in 0..s.len() {
        for j in 0..t.len() {
            if s.chars().nth(i) == t.chars().nth(j) {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = std::cmp::max(dp[i][j + 1], dp[i + 1][j]);
            }
        }
    }
    return dp[s.len()][t.len()];
}


fn main() {
    let s = "abcd";
    let t = "becd";

    let mut dp: Vec<Vec<usize>> = Vec::new();
    for _i in 0..10 {
        let row: Vec<usize> = vec![0; 10];
        dp.push(row);
    }

    let ans = rec(&s, &t, &mut dp);
    println!("{:?}", ans);
}
