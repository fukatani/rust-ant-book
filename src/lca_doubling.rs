fn dfs(
    cur: usize,
    p: usize,
    cur_depth: usize,
    g: &Vec<Vec<usize>>,
    parent: &mut Vec<Vec<usize>>,
    depth: &mut Vec<usize>,
) {
    parent[0][cur] = p;
    depth[cur] = cur_depth;
    for &to in g[cur].iter() {
        if to == p {
            continue;
        }
        dfs(to, cur, cur_depth + 1, g, parent, depth);
    }
}

fn main() {
    let n = 8;
    let mut g = vec![vec![]; n];
    g[0] = vec![1, 2];
    g[1] = vec![3, 4];
    g[2] = vec![5];
    g[4] = vec![6, 7];
    let log_v = 4;
    let mut parent = vec![vec![std::usize::MAX; n]; log_v + 1];
    let mut depth = vec![0usize; n];

    dfs(0, std::usize::MAX, 0, &g, &mut parent, &mut depth);

    for k in 0..log_v {
        for v in 0..n {
            if parent[k][v] == std::usize::MAX {
                parent[k + 1][v] = std::usize::MAX;
            } else {
                parent[k + 1][v] = parent[k][parent[k][v]];
            }
        }
    }

    assert_eq!(1, lca(3, 7, &depth, &parent));
}

fn lca(mut u: usize, mut v: usize, depth: &Vec<usize>, parent: &Vec<Vec<usize>>) -> usize {
    if depth[u] > depth[v] {
        std::mem::swap(&mut u, &mut v);
    }
    for k in 0..parent.len() {
        if (depth[v] - depth[u] >> k) & 1 == 1 {
            v = parent[k][v];
        }
    }
    if u == v {
        return u;
    }
    for k in (0..parent.len()).rev() {
        if parent[k][u] != parent[k][v] {
            u = parent[k][u];
            v = parent[k][v];
        }
    }
    return parent[0][u];
}
