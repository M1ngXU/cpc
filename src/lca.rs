pub struct Lca<const UP: usize = 20> {
    up: Vec<[usize; UP]>,
    tin: Vec<usize>,
    tout: Vec<usize>,
}
impl<const UP: usize> Lca<UP> {
    pub fn new(tree: &Vec<Vec<usize>>) -> Self {
        let mut tin = vec![0; tree.len()];
        let mut tout = vec![0; tree.len()];
        let mut up = vec![[0; UP]; tree.len()];

        fn dfs<const UP: usize>(
            u: usize,
            p: usize,
            tree: &Vec<Vec<usize>>,
            up: &mut Vec<[usize; UP]>,
            tin: &mut Vec<usize>,
            tout: &mut Vec<usize>,
            t: &mut usize,
        ) {
            *t += 1;
            up[u][0] = p;
            tin[u] = *t;
            for i in 1..up[u].len() {
                up[u][i] = up[up[u][i - 1]][i - 1];
            }

            for v in &tree[u] {
                if *v != p {
                    dfs(*v, u, tree, up, tin, tout, t);
                }
            }

            *t += 1;
            tout[u] = *t;
        }

        dfs(0, 0, tree, &mut up, &mut tin, &mut tout, &mut 0);

        Self { up, tin, tout }
    }

    pub fn is_anc(&self, u: usize, v: usize) -> bool {
        self.tin[u] <= self.tin[v] && self.tout[v] <= self.tout[u]
    }

    /// Returns lca + parent[lca]
    pub fn lca(&self, mut u: usize, v: usize) -> (usize, usize) {
        if self.is_anc(u, v) {
            return (u, self.up[u][0]);
        }
        if self.is_anc(v, u) {
            return (v, self.up[v][0]);
        }

        for i in (0..self.up[0].len()).rev() {
            if !self.is_anc(self.up[u][i], v) {
                u = self.up[u][i];
            }
        }
        (self.up[u][0], self.up[u][1])
    }
}
