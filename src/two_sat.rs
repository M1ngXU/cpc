pub struct TwoSatSolver {
    vars: usize,
    vertices: usize,
    edges: Vec<Vec<usize>>,
    edges_transposed: Vec<Vec<usize>>,
}
impl TwoSatSolver {
    pub fn new(vars: usize) -> Self {
        let vertices = vars << 1;
        Self {
            vars,
            vertices,
            edges: vec![vec![]; vertices],
            edges_transposed: vec![vec![]; vertices],
        }
    }

    pub fn solve(&mut self) -> Option<Vec<bool>> {
        fn dfs1(v: usize, used: &mut Vec<bool>, order: &mut Vec<usize>, edges: &Vec<Vec<usize>>) {
            used[v] = true;
            for u in &edges[v] {
                if !used[*u] {
                    dfs1(*u, used, order, edges);
                }
            }
            order.push(v);
        }
        fn dfs2(v: usize, cl: usize, comp: &mut Vec<usize>, edges_transposed: &Vec<Vec<usize>>) {
            comp[v] = cl;
            for u in &edges_transposed[v] {
                if comp[*u] == usize::MAX {
                    dfs2(*u, cl, comp, edges_transposed);
                }
            }
        }

        let mut used = vec![false; self.vertices];
        let mut order = Vec::with_capacity(self.vertices);
        for v in 0..self.vertices {
            if !used[v] {
                dfs1(v, &mut used, &mut order, &self.edges);
            }
        }
        let mut comp = vec![usize::MAX; self.vertices];
        let mut j = 0;
        for i in 0..self.vertices {
            let v = order[self.vertices - 1 - i];
            if comp[v] == usize::MAX {
                dfs2(v, j, &mut comp, &self.edges_transposed);
                j += 1;
            }
        }

        let mut assignment = vec![false; self.vars];
        for i in (0..self.vertices).step_by(2) {
            if comp[i] == comp[i + 1] {
                return None;
            }
            assignment[i >> 1] = comp[i] > comp[i + 1];
        }
        Some(assignment)
    }

    /// Add `((neg_a?) a) or ((neg_b?) b)`
    pub fn add_or(&mut self, a: usize, neg_a: bool, b: usize, neg_b: bool) {
        debug_assert!(a < self.vars, "`a` (`{a}`) out of bounds");
        debug_assert!(b < self.vars, "`b` (`{b}`) out of bounds");
        let [a, b] = [(a, neg_a), (b, neg_b)].map(|(v, n)| (v << 1) ^ (n as usize));
        let (neg_a, neg_b) = (a ^ 1, b ^ 1);
        self.edges[neg_a].push(b);
        self.edges[neg_b].push(a);
        self.edges_transposed[a].push(neg_b);
        self.edges_transposed[b].push(neg_a);
    }

    /// Add `((neg_a?) a) implies ((neg_b?) b)`
    pub fn add_implies(&mut self, a: usize, neg_a: bool, b: usize, neg_b: bool) {
        self.add_or(a, !neg_a, b, neg_b);
    }

    /// Add `((neg_a?) a) nand ((neg_b?) b)`
    pub fn add_nand(&mut self, a: usize, neg_a: bool, b: usize, neg_b: bool) {
        self.add_or(a, !neg_a, b, !neg_b);
    }

    /// Add `((neg_a?) a) nor ((neg_b?) b)`
    pub fn add_nor(&mut self, a: usize, neg_a: bool, b: usize, neg_b: bool) {
        self.force(a, !neg_a);
        self.force(b, !neg_b);
    }

    /// Add `((neg_a?) a) xor ((neg_b?) b)`
    pub fn add_xor(&mut self, a: usize, neg_a: bool, b: usize, neg_b: bool) {
        self.add_or(a, neg_a, b, neg_b);
        self.add_or(a, !neg_a, b, !neg_b);
    }

    /// Add `((neg_a?) a) xnor ((neg_b?) b)`
    pub fn add_xnor(&mut self, a: usize, neg_a: bool, b: usize, neg_b: bool) {
        self.add_or(a, !neg_a, b, neg_b);
        self.add_or(a, neg_a, b, !neg_b);
    }

    /// Force `(neg_a?) a` to have a certain value
    pub fn force(&mut self, a: usize, neg_a: bool) {
        self.add_or(a, neg_a, a, neg_a);
    }
}
