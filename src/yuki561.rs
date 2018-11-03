fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn main() {
    let v:Vec<usize> = read_vec();
    let (n, d) = (v[0], v[1] as i32);
    let mut t = vec![0i32; n];
    let mut k = vec![0i32; n];

    for i in 0..n {
        let v:Vec<i32> = read_vec();
        t[i] = v[0];
        k[i] = v[1];
    }

    let mut t_dp = vec![0; n];
    let mut k_dp = vec![0; n];

    t_dp[0] = t[0];
    k_dp[0] = k[0] - d;

    for i in 0..n - 1 {
        // println!("{:?}", i);
        t_dp[i + 1] = std::cmp::max(t_dp[i], k_dp[i] - d) + t[i + 1];
        k_dp[i + 1] = std::cmp::max(k_dp[i], t_dp[i] - d) + k[i + 1];
    }
    let ans = std::cmp::max(t_dp[n - 1], k_dp[n - 1]);
    println!("{:?}", ans);
}
