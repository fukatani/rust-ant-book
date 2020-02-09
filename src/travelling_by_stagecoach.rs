use std::cmp::*;

#[derive(PartialEq, PartialOrd)]
pub struct Total<T>(pub T);

impl<T: PartialEq> Eq for Total<T> {}

impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, other: &Total<T>) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

fn main() {
    let n = 2;
    let m = 4;
    let a = 2 - 1;
    let b = 1 - 1;
    let t = vec![3.0, 1.0];

    let mut graph = vec![vec![std::f32::MAX; m]; m];
    graph[0][3] = 2.0;
    graph[3][0] = 2.0;
    graph[0][2] = 3.0;
    graph[2][0] = 3.0;
    graph[1][3] = 5.0;
    graph[3][1] = 5.0;
    graph[1][2] = 3.0;
    graph[2][1] = 3.0;

    let mut dp = vec![vec![std::f32::MAX; 4]; 1 << t.len()];
    dp[(1 << t.len()) - 1][a] = 0.0;
    for i in (0..1 << t.len()).rev() {
        for j in 0..m {
            for k in 0..m {
                for ti in 0..n {
                    if i & (1 << ti) == 0 {
                        continue;
                    }
                    if graph[j][k] != std::f32::MAX && dp[i][k] != std::f32::MAX {
                        dp[i & !(1 << ti)][j] = min(
                            Total(dp[i & (!1 << ti)][j]),
                            Total(dp[i][k] + graph[j][k] / t[ti]),
                        ).0;
                    }
                }
            }
        }
    }
    let mut ans = std::f32::MAX;
    for i in (0..1 << t.len()).rev() {
        ans = min(Total(ans), Total(dp[i][b])).0;
    }
    println!("{:?}", ans);
}
