fn dfs_depth(g: &Vec<Vec<usize>>, cur: usize, parent: usize) -> (usize, usize) {
    let mut max_depth = 0;
    let mut max_vertex = cur;
    for &ci in g[cur].iter() {
        if ci == parent {
            continue;
        }
        let (depth, vertex) = dfs_depth(g, ci, cur);
        if max_depth < depth {
            max_depth = depth;
            max_vertex = vertex;
        }
    }
    (1 + max_depth, max_vertex)
}

fn tree_diameter(g: &Vec<Vec<usize>>) -> usize {
    let (_, vertex1) = dfs_depth(&g, 0, g.len());
    let (depth, _) = dfs_depth(&g, vertex1, g.len());
    depth
}

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
        dfs_depth(g, ci, cur, cur_cost + cost, depth);
    }
}

fn tree_diameter2(g: &Vec<Vec<(usize, i64)>>) -> (usize, usize) {
    let mut depth = vec![0; g.len()];
    dfs_depth(&g, 0, g.len(), 0, &mut depth);
    let mut vertex1 = 0;
    for i in 0..g.len() {
        if depth[vertex1] < depth[i] {
            vertex1 = i;
        }
    }
    let mut depth = vec![0; g.len()];
    dfs_depth(&g, vertex1, g.len(), 0, &mut depth);
    let mut vertex2 = 0;
    for i in 0..g.len() {
        if depth[vertex2] < depth[i] {
            vertex2 = i;
        }
    }
    (vertex1, vertex2)
}

fn main() {
    let mut g = vec![vec![]; 6];
    let pairs = vec![(1, 2), (2, 3), (2, 4), (4, 6), (5, 6)];
    for (a, b) in pairs.iter().map(|&(a, b)| (a - 1, b - 1)) {
        g[a].push(b);
        g[b].push(a);
    }
    assert_eq!(tree_diameter(&g), 5);

    let mut g = vec![vec![]; 7];
    let pairs = vec![(1, 7), (7, 4), (3, 4), (7, 5), (6, 3), (2, 1)];
    for (a, b) in pairs.iter().map(|&(a, b)| (a - 1, b - 1)) {
        g[a].push(b);
        g[b].push(a);
    }
    assert_eq!(tree_diameter(&g), 6);
}
