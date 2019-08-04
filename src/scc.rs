type Graph = Vec<Vec<usize>>;

fn dfs(v: usize, used: &mut Vec<bool>, g: &Graph, vs: &mut Vec<usize>) {
    used[v] = true;
    for &to in g[v].iter() {
        if !used[to] {
            dfs(to, used, g, vs);
        }
    }
    vs.push(v);
}

fn rdfs(v: usize, k: usize, used: &mut Vec<bool>, rg: &Graph, cmp: &mut Vec<usize>) {
    used[v] = true;
    cmp[v] = k;
    for &to in rg[v].iter() {
        if !used[to] {
            rdfs(to, k, used, rg, cmp);
        }
    }
}

fn add_edge(from: usize, to: usize, g: &mut Graph, rg: &mut Graph) {
    g[from].push(to);
    rg[to].push(from);
}

fn main() {
    let n = 12;
    let mut g = vec![vec![]; n];
    let mut rg = vec![vec![]; n];

    add_edge(11, 10, &mut g, &mut rg);
    add_edge(10, 7, &mut g, &mut rg);
    add_edge(10, 9, &mut g, &mut rg);
    add_edge(9, 8, &mut g, &mut rg);
    add_edge(8, 7, &mut g, &mut rg);

    add_edge(7, 9, &mut g, &mut rg);
    add_edge(8, 6, &mut g, &mut rg);
    add_edge(6, 5, &mut g, &mut rg);
    add_edge(5, 4, &mut g, &mut rg);
    add_edge(4, 6, &mut g, &mut rg);

    add_edge(5, 2, &mut g, &mut rg);
    add_edge(5, 3, &mut g, &mut rg);
    add_edge(3, 2, &mut g, &mut rg);
    add_edge(3, 0, &mut g, &mut rg);
    add_edge(1, 2, &mut g, &mut rg);
    add_edge(2, 1, &mut g, &mut rg);

    let cmp = scc(&g, &rg);
    println!("{:?}", cmp);
}

fn scc(g: &Graph, rg: &Graph) -> Vec<usize> {
    let mut used = vec![false; g.len()];
    let mut vs = vec![];
    for v in 0..g.len() {
        if !used[v] {
            dfs(v, &mut used, &g, &mut vs);
        }
    }
    println!("{:?}", vs);
    let mut used = vec![false; g.len()];
    let mut cmp = vec![0usize; g.len()];
    let mut k = 0;
    for &i in vs.iter().rev() {
        if !used[i] {
            rdfs(i, k, &mut used, &rg, &mut cmp);
            k += 1;
        }
    }
    cmp
}
