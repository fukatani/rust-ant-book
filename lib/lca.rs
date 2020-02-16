struct LCA {
    depth: Vec<usize>,
    parent: Vec<Vec<Option<usize>>>,
    log_v: usize,
}

impl LCA {
    fn new(g: &Vec<Vec<usize>>) -> LCA {
        LCA::with_root(g, 0)
    }

    fn with_root(g: &Vec<Vec<usize>>, root: usize) -> LCA {
        let n = g.len();
        let log_v = (1..).find(|i| 1usize << i > n).unwrap();
        let mut lca = LCA {
            depth: vec![0usize; n],
            parent: vec![vec![None; n]; 20 + 1], // support 2 ^ 20
            log_v: log_v,
        };
        lca.dfs(root, None, 0, &g);
        lca.doubling(g.len());
        lca
    }

    fn doubling(&mut self, n: usize) {
        for k in 0..self.log_v {
            for v in 0..n {
                if let Some(parent_v) = self.parent[k][v] {
                    self.parent[k + 1][v] = self.parent[k][parent_v];
                }
            }
        }
    }

    fn dfs(&mut self, cur: usize, p: Option<usize>, cur_depth: usize, g: &Vec<Vec<usize>>) {
        self.parent[0][cur] = p;
        self.depth[cur] = cur_depth;
        for &to in g[cur].iter() {
            if Some(to) == p {
                continue;
            }
            self.dfs(to, Some(cur), cur_depth + 1, g);
        }
    }

    fn lca(&self, mut u: usize, mut v: usize) -> usize {
        if self.depth[u] > self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }
        while self.depth[v] != self.depth[u] {
            v = self.parent[(self.depth[v] - self.depth[u]).trailing_zeros() as usize][v].unwrap();
        }
        if u == v {
            return u;
        }
        for k in (0..self.parent.len()).rev() {
            if self.parent[k][u] != self.parent[k][v] {
                u = self.parent[k][u].unwrap();
                v = self.parent[k][v].unwrap();
            }
        }
        self.parent[0][u].unwrap()
    }
}

fn main() {
    let n = 8;
    let mut g = vec![vec![]; n];
    g[0] = vec![1, 2];
    g[1] = vec![3, 4];
    g[2] = vec![5];
    g[4] = vec![6, 7];
    let lca = LCA::new(&g);

    assert_eq!(1, lca.lca(3, 7));
}
