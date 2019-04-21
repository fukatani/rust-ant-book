#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;

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

fn dfs(u: usize, prev: usize, g: &Vec<Vec<usize>>, sz: &mut Vec<usize>, centroid: &mut Vec<usize>) {
    sz[u] = 1;
    let mut is_centroid = true;
    for &v in g[u].iter() {
        if v != prev {
            dfs(v, u, g, sz, centroid);
            sz[u] += sz[v];
            if sz[v] > g.len() / 2 {
                is_centroid = false;
            }
        }
    }
    if g.len() - sz[u] > g.len() / 2 {
        is_centroid = false;
    }
    if is_centroid {
        centroid.push(u);
    }
}

fn centroid(g: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut centroid = Vec::new();
    let mut sz = vec![0usize; g.len()];
    dfs(0, std::usize::MAX, g, &mut sz, &mut centroid);
    centroid
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
    let n: usize = read();
    let mut edges = HashMap::<usize, HashMap<usize, u64>>::new();
    let mut simple_edges = vec![Vec::new(); n];
    for i in 0..n - 1 {
        let v = read_vec::<u64>();
        let a = v[0] as usize - 1;
        let b = v[1] as usize - 1;
        edges.entry(a).or_insert(HashMap::new()).insert(b, v[2]);
        edges.entry(b).or_insert(HashMap::new()).insert(a, v[2]);
        simple_edges[a].push(b);
        simple_edges[b].push(a);
    }
    if n == 2 {
        println!("{}", edges[&0][&1]);
        return;
    }

    let centroid = centroid(&simple_edges);

    let root = centroid[0];
    let mut tree: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut parent_dict: HashMap<usize, usize> = HashMap::new();
    make_tree(root, n, &simple_edges, &mut tree, &mut parent_dict);
    let mut topological_sorted_indexes = vec![root];
    topological_dfs(root, &tree, &mut topological_sorted_indexes);

    let mut rank = vec![0u64; n];
    for &i in topological_sorted_indexes.iter().rev() {
        if tree[i].is_empty() {
            rank[i] = 1;
            continue;
        }
        rank[i] = tree[i].iter().map(|&x| rank[x]).sum::<u64>() + 1;
    }

    let mut ans;
    if centroid.len() == 1 {
        ans = (0..n)
            .map(|x| {
                if x == root {
                    0
                } else {
                    2 * rank[x] * edges[&x][&parent_dict[&x]]
                }
            })
            .sum::<u64>();
        let mut lightest_end = (0..n)
            .map(|x| {
                if x == root || parent_dict[&x] != root {
                    std::u64::MAX
                } else {
                    edges[&x][&parent_dict[&x]]
                }
            })
            .min()
            .unwrap();
        ans -= lightest_end;
    } else {
        ans = (0..n)
            .map(|x| {
                if x == root || x == centroid[1] {
                    0
                } else {
                    2 * rank[x] * edges[&x][&parent_dict[&x]]
                }
            })
            .sum::<u64>();
        ans += (n as u64 - 1) * edges[&centroid[0]][&centroid[1]];
    }

    println!("{}", ans);
}
