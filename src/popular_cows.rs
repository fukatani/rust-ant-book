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
    let n = 3;
    let mut g = vec![vec![]; n];
    let mut rg = vec![vec![]; n];

    add_edge(0, 1, &mut g, &mut rg);
    add_edge(1, 0, &mut g, &mut rg);
    add_edge(1, 2, &mut g, &mut rg);

    let cmp = scc(&g, &rg);

    let k = *cmp.iter().max().unwrap();
    let mut ans = 0;
    let mut u = 0;
    for i in 0..n {
        if cmp[i] == k {
            u = i;
            ans += 1;
        }
    }

    let mut used = vec![false; g.len()];
    let mut cmp = vec![0usize; g.len()];
    rdfs(u, 0, &mut used, &rg, &mut cmp);

    for i in 0..n {
        // found unreaceable vertex
        if !used[i] {
            ans = 0;
            break;
        }
    }
    println!("{}", ans);
}

fn scc(g: &Graph, rg: &Graph) -> Vec<usize> {
    let mut used = vec![false; g.len()];
    let mut vs = vec![];
    for v in 0..g.len() {
        if !used[v] {
            dfs(v, &mut used, &g, &mut vs);
        }
    }

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
