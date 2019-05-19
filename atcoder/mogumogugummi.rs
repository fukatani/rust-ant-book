#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::cmp::*;
use std::collections::*;
use std::io::Write;

#[allow(unused_macros)]
macro_rules! debug {
    ($($e:expr),*) => {
        #[cfg(debug_assertions)]
        $({
            let (e, mut err) = (stringify!($e), std::io::stderr());
            writeln!(err, "{} = {:?}", e, $e).unwrap()
        })*
    };
}

fn main() {
    let n = read::<usize>();
    let mut edges = vec![HashMap::new(); n];
    let mut simple_edges = vec![Vec::new(); n];
    for a in 1..n {
        let v = read_vec::<u64>();
        let b = v[0] as usize;
        edges[a].insert(b, v[1]);
        edges[b].insert(a, v[1]);
        simple_edges[a].push(b);
        simple_edges[b].push(a);
    }

    let root = 0usize;
    let mut tree: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut parent_dict: HashMap<usize, usize> = HashMap::new();
    make_tree(root, n, &simple_edges, &mut tree, &mut parent_dict);
    let mut topological_sorted_indexes = vec![root];
    topological_dfs(root, &tree, &mut topological_sorted_indexes);

    let mut dp = vec![vec![std::u64::MAX; n + 1]; n + 1];
    for i in 0..n {
        dp[i][0] = 0;
    }
    for &ti in topological_sorted_indexes.iter().rev() {
        if ti == root {
            continue;
        }
        let parent = parent_dict[&ti];
        let edge_hard = edges[ti][&parent];

        for &ci in tree[ti].iter() {
            for j in (0..n + 1).rev() {
                if dp[ti][j] == std::u64::MAX {
                    continue;
                }
                for i in (0..min(n - j + 1, dp[ci].len())).rev() {
                    if dp[ci][i] == std::u64::MAX {
                        continue;
                    }

                    if dp[ci][i] + dp[ti][j] <= edge_hard {
                        dp[ti][i + j] = min(dp[ti][i + j], dp[ci][i] + dp[ti][j]);
                        // debug!(i, j, dp[ti][i + j], dp[ci][i], dp[ti][j]);
                    }
                }
            }
            // debug!(ci, dp[ti]);
        }
        for i in (0..n + 1).rev() {
            if dp[ti][i] != std::u64::MAX {
                dp[ti][i + 1] = edge_hard;
                dp[ti].resize(i + 2, 0);
                break;
            }
        }
        // debug!(ti, dp[ti]);
        // debug!("");
    }

    // calc root
    // debug!(dp);

    let mut ans = 1;
    for &ci in tree[0].iter() {
        let mut temp = 0;
        for i in 0..dp[ci].len() {
            if dp[ci][i] != std::u64::MAX {
                temp = i;
            }
        }
        ans += temp;
    }
    println!("{:?}", ans);
}

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

fn make_tree(
    cur_idx: usize,
    parent_idx: usize,
    edges: &Vec<Vec<usize>>,
    tree: &mut Vec<Vec<usize>>,
    parent_dict: &mut HashMap<usize, usize>,
) {
    for child_idx in edges[cur_idx].iter() {
        if *child_idx == parent_idx {
            continue;
        }
        tree[cur_idx].push(*child_idx);
        parent_dict.insert(*child_idx, cur_idx);
        make_tree(*child_idx, cur_idx, edges, tree, parent_dict);
    }
}

fn topological_dfs(cur_idx: usize, tree: &Vec<Vec<usize>>, result: &mut Vec<usize>) {
    for child_idx in tree[cur_idx].iter() {
        result.push(*child_idx);
        topological_dfs(*child_idx, tree, result);
    }
}
