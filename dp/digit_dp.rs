// https://pekempey.hatenablog.com/entry/2015/12/09/000603

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let n: String = read();
    let mut dp = vec![vec![vec![vec![0; 3]; 2]; 2]; n.len() + 1];
    let big_int = 1000_000_000 + 7;
    dp[0][0][0][0] = 1;  // pos, less, has3, mod3

    for (i, c) in n.chars().enumerate() {
        for j in 0..2 {
            for k in 0..2 {
                for m in 0..3 {
                    let lim = if j == 1 { 9 } else { c.to_digit(10).unwrap() };
                    for l in 0..lim + 1 {
                        let idx1 = if j == 1 || l < lim { 1 } else { 0 };
                        let idx2 = if k == 1 || l == 3 { 1 } else { 0 };
                        let idx3: usize = (m + l as usize) % 3;
                        dp[i + 1][idx1][idx2][idx3] += dp[i][j][k][m];
                        dp[i + 1][idx1][idx2][idx3] %= big_int;
                    }
                }
            }
        }
    }
    println!("{:?}", dp);

    let mut ans = 0;
    for j in 0..2 {
        ans += dp[n.len()][j][1][0];
        ans %= big_int;
    }
    println!("{:?}", ans);
}
