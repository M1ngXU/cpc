pub struct LazySegmentTreeBeats {
    pub n: usize,
    pub tree: Vec<SegmentTreeType>,
    lazy: Vec<Option<UpdateType>>,
}
impl LazySegmentTreeBeats {
    pub fn new(initial: &[SegmentTreeType]) -> Self {
        let n = input.len().next_power_of_two();
        let mut tree = vec![DEFAULT_SEGMENT_TREE_TYPE; 2 * n - 1];
        tree[n - 1..n - 1 + input.len()].clone_from_slice(input);
        for i in (0..n - 1).rev() {
            tree[i] = transform(&tree[2 * i + 1], &tree[2 * i + 2]);
        }
        Self {
            tree,

            lazy: vec![None; n - 1],
            n,
        }
    }

    /// Query the segment tree for the range [l, r) and update lazy stuff.
    pub fn query(&mut self, l: usize, r: usize) -> Option<SegmentTreeType> {
        self.query_rec(0, 0, self.n, l, r)
    }

    fn push_down(&mut self, i: usize) {
        if let Some(v) = self.lazy[i].take() {
            for j in 1..=2 {
                let idx = (i << 1) + j;
                if idx < self.tree.len() {
                    self.tree[idx] = update(&self.tree[idx], &v, self.len_at(idx));
                    self.add_lazy(idx, &v);
                }
            }
        }
    }

    fn query_rec(
        &mut self,
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

        self.push_down(i);

        let m = (il + ir) >> 1;
        let left = self.query_rec(2 * i + 1, il, m, l, r);
        let right = self.query_rec(2 * i + 2, m, ir, l, r);
        match (left, right) {
            (Some(l), Some(r)) => Some(transform(&l, &r)),
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (None, None) => None,
        }
    }

    fn add_lazy(&mut self, i: usize, v: &UpdateType) {
        if i < self.lazy.len() {
            if let Some(l) = self.lazy[i].take() {
                self.lazy[i] = Some(todo!("aggregate"));
            } else {
                self.lazy[i] = Some(v.clone());
            }
        }
    }

    /// Lazily update all values in [l; r) to v
    pub fn update(&mut self, l: usize, r: usize, v: UpdateType) {
        self.update_rec(0, 0, self.n, l, r, v);
    }

    fn update_rec(&mut self, i: usize, il: usize, ir: usize, l: usize, r: usize, v: UpdateType) {
        if ir <= l || r <= il || todo!("breakCondition") {
            return;
        }
        if l <= il && ir <= r && todo!("tagCondition") {
            self.tree[i] = update(&self.tree[i], &v, self.len_at(i));
            self.add_lazy(i, &v);
            if i != 0 {
                let mut i = i;
                while i > 0 {
                    i = (i - 1) >> 1;
                    self.tree[i] = transform(&self.tree[(i << 1) + 1], &self.tree[(i << 1) + 2]);
                }
            }
            return;
        }

        self.push_down(i);

        let m = (il + ir) >> 1;
        self.update_rec(2 * i + 1, il, m, l, r, v.clone());
        self.update_rec(2 * i + 2, m, ir, l, r, v.clone());
    }

    fn len_at(&self, i: usize) -> usize {
        self.n >> (i + 1).ilog2()
    }
}

pub type SegmentTreeType = compile_error!("TagType");
pub type UpdateType = compile_error!("UpdateType");
const DEFAULT_SEGMENT_TREE_TYPE: SegmentTreeType = 0;

pub fn transform(lhs: &SegmentTreeType, rhs: &SegmentTreeType) -> SegmentTreeType {
    todo!()
}
pub fn update(old: &SegmentTreeType, update: &UpdateType, segment_len: usize) -> SegmentTreeType {
    todo!()
}
