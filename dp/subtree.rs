use std::collections::HashMap;

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

fn main() {
    let v = read_vec::<u64>();
    let (n, m) = (v[0], v[1]);
    let n = n as usize;
    if n == 1 {
        println!("1");
        return;
    }
    let mut edges: Vec<Vec<usize>> = vec![Vec::new(); 2 * (n - 1)];
    for _ in 0..n - 1 {
        let v = read_vec::<usize>();
        let (a, b) = (v[0] - 1, v[1] - 1);
        edges[a].push(b);
        edges[b].push(a);
    }

    let mut tree: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut parent_dict: HashMap<usize, usize> = HashMap::new();
    make_tree(0, n, &edges, &mut tree, &mut parent_dict);
    let mut topological_sorted_indexes = vec![0];
    topological_dfs(0, &tree, &mut topological_sorted_indexes);

    let mut dp = vec![1u64; n];
    let mut dp_l = vec![Vec::new(); n];
    let mut dp_r = vec![Vec::new(); n];

    for &ti in topological_sorted_indexes.iter().rev() {
        if !tree[ti].is_empty() {
            for &idx in tree[ti].iter() {
                dp[ti] *= dp[idx] + 1;
                dp[ti] %= m;
            }

            dp_l[ti] = vec![1u64; tree[ti].len()];
            dp_l[ti][0] = dp[tree[ti][0]] + 1;
            for ci in 1..tree[ti].len() {
                let idx = tree[ti][ci];
                dp_l[ti][ci] = dp_l[ti][ci - 1] * (dp[idx] + 1);
                dp_l[ti][ci] %= m;
            }

            dp_r[ti] = vec![1u64; tree[ti].len()];
            dp_r[ti][0] = dp[tree[ti][tree[ti].len() - 1]] + 1;
            for ci in 1..tree[ti].len() {
                let idx = tree[ti][tree[ti].len() - 1 - ci];
                dp_r[ti][ci] = dp_r[ti][ci - 1] * (dp[idx] + 1);
                dp_r[ti][ci] %= m;
            }
        }
    }

    let mut dp_i = vec![1; n];
    dp_i[0] = 0;
    for &ti in topological_sorted_indexes.iter() {
        for (ci, &cidx) in tree[ti].iter().enumerate() {
            dp_i[cidx] = dp_i[ti] + 1;
            if ci > 0 {
                dp_i[cidx] *= dp_l[ti][ci - 1];
                dp_i[cidx] %= m;
            }
            if ci < tree[ti].len() - 1 {
                dp_i[cidx] *= dp_r[ti][tree[ti].len() - 2 - ci];
                dp_i[cidx] %= m;
            }
        }
    }

    // println!("{:?}", dp);
    // println!("{:?}", dp_i);
    for i in 0..n {
        println!("{:?}", dp[i] * (dp_i[i] + 1) % m);
    }
}
