fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let s: String = read();
    let t: String = read();
    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();

    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    for i in 0..s.len() {
        for j in 0..t.len() {
            if s[i] == t[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = std::cmp::max(dp[i][j + 1], dp[i + 1][j]);
            }
        }
    }
    // println!("{:?}", dp);

    let mut ans: Vec<char> = Vec::new();
    let mut i = s.len();
    let mut j = t.len();
    while i > 0 {
        if dp[i][j] == dp[i - 1][j - 1] + 1 {
            ans.push(s[i - 1]);
            i -= 1;
            j -= 1;
        } else {
            i -= 1;
        }
    }

    ans.reverse();
    for ch in ans {
        print!("{}", ch);
    }
    println!("");
}
