struct LCA {
    depth: Vec<usize>,
    parent: Vec<Vec<usize>>,
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
            parent: vec![vec![std::usize::MAX; n]; 20 + 1], // support 2 ^ 20
            log_v: log_v,
        };
        lca.dfs(root, std::usize::MAX, 0, &g);
        lca.doubling(g.len());
        lca
    }

    fn doubling(&mut self, n: usize) {
        for k in 0..self.log_v {
            for v in 0..n {
                if self.parent[k][v] == std::usize::MAX {
                    self.parent[k + 1][v] = std::usize::MAX;
                } else {
                    self.parent[k + 1][v] = self.parent[k][self.parent[k][v]];
                }
            }
        }
    }

    fn dfs(&mut self, cur: usize, p: usize, cur_depth: usize, g: &Vec<Vec<usize>>) {
        self.parent[0][cur] = p;
        self.depth[cur] = cur_depth;
        for &to in g[cur].iter() {
            if to == p {
                continue;
            }
            self.dfs(to, cur, cur_depth + 1, g);
        }
    }

    fn lca(&self, mut u: usize, mut v: usize) -> usize {
        if self.depth[u] > self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }
        for k in 0..self.parent.len() {
            if (self.depth[v] - self.depth[u] >> k) & 1 == 1 {
                v = self.parent[k][v];
            }
        }
        if u == v {
            return u;
        }
        for k in (0..self.parent.len()).rev() {
            if self.parent[k][u] != self.parent[k][v] {
                u = self.parent[k][u];
                v = self.parent[k][v];
            }
        }
        self.parent[0][u]
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
