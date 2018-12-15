fn main() {
    let m = 3usize;
    let p = 0.75;
    let x = 600000;

    let state_size = 1 << m as usize + 1;
    let mut dp = vec![vec![0.0; state_size]; m + 1];

    dp[0][state_size - 1] = 1.0;

    for round in 0..m {
        for state in 0..state_size {
            let jub = std::cmp::min(state, state_size - 1 - state);
            let mut t = 0.0f32;
            for j in 0..jub + 1 {
                t = t.max(p * dp[round][state + j] + (1.0 - p) * dp[round][state - j]);
            }
            dp[round + 1][state] = t;
        }
    }
    println!("{:?}", dp);
    let start_state = x * state_size / 1000000;
    println!("{:?}", dp[m][start_state]);
}
