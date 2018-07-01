static mut N: usize = 0;

fn rec(i:usize, j:usize, w: &Vec<usize>, v: &Vec<usize>, dp: &mut Vec<Vec<usize>>) -> usize {
    if dp[i][j] > 0 {
        return dp[i][j];
    }
    unsafe {
        if i == N {
            dp[i][j] = 0;
        } else if j < w[i] {
            dp[i][j] = rec(i + 1, j, w, v, dp);
        } else {
            dp[i][j] = std::cmp::max(rec(i + 1, j, w, v, dp), rec(i + 1, j - w[i], w, v, dp) + v[i]);
        }
    }
    return dp[i][j];
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

    let ans = rec(0, 5, &w, &v, &mut dp);
    println!("{:?}", ans);
}
