pub struct PersistentSegmentTree {
    roots: Vec<usize>,
    nodes: Vec<(usize, usize, SegmentTreeType)>,
    n: usize,
}
impl PersistentSegmentTree {
    pub fn build(initial_values: &[SegmentTreeType]) -> Self {
        let n = initial_values.len();
        let mut nodes = vec![];
        let root = Self::build_recursive(0, n, initial_values, &mut nodes);
        let roots = vec![root];
        Self { roots, nodes, n }
    }
    fn build_recursive(
        i: usize,
        j: usize,
        initial_values: &[SegmentTreeType],
        nodes: &mut Vec<(usize, usize, SegmentTreeType)>,
    ) -> usize {
        if j - i == 1 {
            let node = nodes.len();
            nodes.push((usize::MAX, usize::MAX, initial_values[i].clone()));
            node
        } else {
            let m = (i + j) / 2;
            let left = Self::build_recursive(i, m, initial_values, nodes);
            let right = Self::build_recursive(m, j, initial_values, nodes);
            let node = nodes.len();
            nodes.push((left, right, transform(&nodes[left].2, &nodes[right].2)));
            node
        }
    }

    /// Returns the "pointer" to the new root
    pub fn update(&mut self, root: usize, i: usize, value: UpdateType) -> usize {
        let new_root = self.update_recursive(self.roots[root], 0, self.n, i, value);
        self.roots.push(new_root);
        self.roots.len() - 1
    }

    fn update_recursive(
        &mut self,
        node: usize,
        left: usize,
        right: usize,
        index: usize,
        value: UpdateType,
    ) -> usize {
        if right - left == 1 {
            let new_node = self.nodes.len();
            self.nodes
                .push((usize::MAX, usize::MAX, update(&self.nodes[node].2, &value)));
            new_node
        } else {
            let m = (left + right) / 2;
            let (l, r, _) = &self.nodes[node];
            let mut new_left = *l;
            let mut new_right = *r;
            if index < m {
                new_left = self.update_recursive(*l, left, m, index, value);
            } else {
                new_right = self.update_recursive(*r, m, right, index, value);
            }
            let new_value = transform(&self.nodes[new_left].2, &self.nodes[new_right].2);
            let new_node = self.nodes.len();
            self.nodes.push((new_left, new_right, new_value));
            new_node
        }
    }
    pub fn query(&self, root: usize, i: usize, j: usize) -> SegmentTreeType {
        self.query_recursive(self.roots[root], 0, self.n, i, j)
    }
    fn query_recursive(
        &self,
        node: usize,
        left: usize,
        right: usize,
        i: usize,
        j: usize,
    ) -> SegmentTreeType {
        if i <= left && right <= j {
            self.nodes[node].2.clone()
        } else {
            let m = (left + right) / 2;
            let (left_child, right_child, _) = self.nodes[node];
            if j <= m {
                self.query_recursive(left_child, left, m, i, j)
            } else if m <= i {
                self.query_recursive(right_child, m, right, i, j)
            } else {
                let left_value = self.query_recursive(left_child, left, m, i, j);
                let right_value = self.query_recursive(right_child, m, right, i, j);
                transform(&left_value, &right_value)
            }
        }
    }
}

pub type SegmentTreeType = compile_error!("SegmentTreeType");
pub type UpdateType = compile_error!("UpdateType");

pub fn transform(lhs: &SegmentTreeType, rhs: &SegmentTreeType) -> SegmentTreeType {
    todo!()
}
pub fn update(old: &SegmentTreeType, update: &UpdateType) -> SegmentTreeType {
    todo!()
}
