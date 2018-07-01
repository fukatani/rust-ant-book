static mut N: usize = 0;
static W: usize = 5;

fn rec(w: &Vec<usize>, v: &Vec<usize>, dp: &mut Vec<Vec<usize>>) -> usize {
    unsafe {
        for i in 0..N {
            let i:usize = N - 1 - i;
            for j in 0..W+1 {
                if j < w[i] {
                    dp[i][j] = dp[i + 1][j];
                } else {
                    dp[i][j] = std::cmp::max(dp[i + 1][j], dp[i + 1][j - w[i]] + v[i]);
                    println!("{0}, {1}, {2}", i, j, dp[i][j]);
                }
            }
        }
    }
    return dp[0][W];
}


fn main() {
    unsafe {
        N = 4;
    }
    let w: Vec<usize> = vec![2 ,1 ,3, 2];
    let v: Vec<usize> = vec![3, 2, 4, 2];
    let mut dp: Vec<Vec<usize>> = Vec::new();
    for _i in 0..10 {
        let row: Vec<usize> = vec![0; 10];
        dp.push(row);
    }

    let ans = rec(&w, &v, &mut dp);
    println!("{:?}", ans);
}
