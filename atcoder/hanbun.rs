const MOD: u64 = 1000000007;

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
    let v = read_vec::<usize>();
    let (N, K) = (v[0], v[1]);
    let a = read_vec::<u64>();
    let mut A = vec![0; N];
    for i in 0..N {
        let mut num = a[i];
        let mut j = 0;
        while num != 0 {
            j += 1;
            num /= 2;
        }
        A[i] = j as usize;
    }
    let K = std::cmp::min(K, A.iter().sum::<usize>());
    let mut dp = vec![vec![vec![0u64; 2]; K + 1]; N + 1];

    dp[0][0][0] = 1;
    for i in 0..N {
        for j in 0..K + 1 {
            // 0 -> 0
            for k in 0..A[i] {
                if j + k > K {
                    break;
                }
                dp[i+1][j+k][0] += dp[i][j][0];
                dp[i+1][j+k][0] %= MOD;
            }

            // 0 -> 1
            if j + A[i] <= K {
                dp[i+1][j+A[i]][1] += dp[i][j][0];
                dp[i+1][j+A[i]][1] %= MOD;
            }

            // 1 -> 0
            for k in 0..A[i] + 1 {
                if j + k > K {
                    break;
                }
                dp[i+1][j+k][1] += dp[i][j][1];
                dp[i+1][j+k][1] %= MOD;
            }
        }
    }

    // 0
    let mut res = dp[N][K][0];
    
    // 1
    for k in 0..K + 1 {
        res += dp[N][k][1];
        res %= MOD;
    }
    println!("{}", res);
}

