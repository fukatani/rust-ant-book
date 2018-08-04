fn solve(w: Vec<usize>, v: Vec<usize>, max_w: usize) -> usize {
    let mut dp = vec![0; max_w + 1];
    for i in 0..w.len() {
        for j in 0..max_w + 1 {
            if j < w[i] {
                dp[j] = dp[j];
            } else {
                dp[j] = std::cmp::max(dp[j], dp[j - w[i]] + v[i]);
            }
        }
    }
    dp[max_w]
}

fn main() {
    let w: Vec<usize> = vec![3, 4, 2];
    let v: Vec<usize> = vec![4, 5, 3];
    let max_w: usize = 7;
    println!("{:?}", solve(w, v, max_w));
}
