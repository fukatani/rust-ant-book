fn dfs_depth2(
    g: &Vec<Vec<(usize, i64)>>,
    cur: usize,
    parent: usize,
    cur_cost: i64,
    depth: &mut Vec<i64>,
) {
    depth[cur] = cur_cost;
    for &(ci, cost) in g[cur].iter() {
        if ci == parent {
            continue;
        }
        dfs_depth2(g, ci, cur, cur_cost + cost, depth);
    }
}

fn tree_diameter2(g: &Vec<Vec<(usize, i64)>>) -> (usize, usize, i64) {
    let mut depth = vec![0; g.len()];
    dfs_depth2(&g, 0, g.len(), 0, &mut depth);
    let mut vertex1 = 0;
    for i in 0..g.len() {
        if depth[vertex1] < depth[i] {
            vertex1 = i;
        }
    }
    let mut depth = vec![0; g.len()];
    dfs_depth2(&g, vertex1, g.len(), 0, &mut depth);
    let mut vertex2 = 0;
    for i in 0..g.len() {
        if depth[vertex2] < depth[i] {
            vertex2 = i;
        }
    }
    (vertex1, vertex2, depth[vertex2])
}

fn main() {
    let mut g = vec![vec![]; 6];
    let pairs = vec![(1, 2), (2, 3), (2, 4), (4, 6), (5, 6)];
    for (a, b) in pairs.iter().map(|&(a, b)| (a - 1, b - 1)) {
        g[a].push(b);
        g[b].push(a);
    }
    assert_eq!(tree_diameter2(&g).2, 5);

    let mut g = vec![vec![]; 7];
    let pairs = vec![(1, 7), (7, 4), (3, 4), (7, 5), (6, 3), (2, 1)];
    for (a, b) in pairs.iter().map(|&(a, b)| (a - 1, b - 1)) {
        g[a].push(b);
        g[b].push(a);
    }
    assert_eq!(tree_diameter2(&g).2, 6);
}
