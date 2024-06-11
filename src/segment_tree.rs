pub struct SegmentTree {
    pub tree: Vec<SegmentTreeType>,
}
impl SegmentTree {
    pub fn new(input: &[SegmentTreeType]) -> Self {
        let n = input.len().next_power_of_two();
        let mut tree = vec![DEFAULT_SEGMENT_TREE_TYPE; 2 * n - 1];
        tree[n - 1..n - 1 + input.len()].clone_from_slice(input);
        for i in (0..n - 1).rev() {
            tree[i] = transform(&tree[2 * i + 1], &tree[2 * i + 2]);
        }
        Self { tree }
    }

    /// Query the segment tree for the range [l, r).
    pub fn query(&self, l: usize, r: usize) -> Option<SegmentTreeType> {
        self.query_rec(0, 0, (self.tree.len() + 1) >> 1, l, r)
    }

    fn query_rec(
        &self,
        i: usize,
        il: usize,
        ir: usize,
        l: usize,
        r: usize,
    ) -> Option<SegmentTreeType> {
        if ir <= l || r <= il {
            return None;
        }
        if l <= il && ir <= r {
            return Some(self.tree[i].clone());
        }
        let m = (il + ir) >> 1;
        let left = self.query_rec(2 * i + 1, il, m, l, r);
        let right = self.query_rec(2 * i + 2, m, ir, l, r);
        match (left, right) {
            (Some(left), Some(right)) => Some(transform(&left, &right)),
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (None, None) => None,
        }
    }

    pub fn update(&mut self, i: usize, v: SegmentTreeType) {
        let n = (self.tree.len() + 1) / 2;
        let mut i = i + n - 1;
        self.tree[i] = v;
        while i > 0 {
            i = (i - 1) / 2;
            self.tree[i] = transform(&self.tree[2 * i + 1], &self.tree[2 * i + 2]);
        }
    }
}

pub type SegmentTreeType = compile_error!("SegmenTreeType");
const DEFAULT_SEGMENT_TREE_TYPE: SegmentTreeType = 0;

pub fn transform(lhs: &SegmentTreeType, rhs: &SegmentTreeType) -> SegmentTreeType {
    todo!()
}
