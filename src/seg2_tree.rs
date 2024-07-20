pub struct SegmentTree {
    pub tree: Vec<SegmentTreeType>,
    pub n: usize
}
impl SegmentTree {
    pub fn new(input: &[SegmentTreeType]) -> Self {
        let n = input.len();
        let mut tree = vec![DEFAULT_SEGMENT_TREE_TYPE; n << 1];
        tree[n..n + input.len()].clone_from_slice(input);
        for i in (1..n).rev() {
            tree[i] = transform(&tree[i << 1], &tree[(i << 1) | 1]);
        }
        Self { tree, n }
    }

    /// Query the segment tree for the range [l, r).
    pub fn query(&self, mut l: usize, mut r: usize) -> Option<SegmentTreeType> {
        let mut res: Option<SegmentTreeType> = None;
        l += self.n;
        r += self.n;
        while l < r {
            if l & 1 == 1 {
                res = Some(if let Some(v) = res {
                    transform(&v, &self.tree[l])
                } else {
                    self.tree[l]
                });
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                res = Some(if let Some(v) = res {
                    transform(&self.tree[r], &v)
                } else {
                    self.tree[r]
                });
            }
            l >>= 1;
            r >>= 1;
        }
        res
    }

    pub fn update(&mut self, i: usize, v: SegmentTreeType) {
        let n = (self.tree.len() + 1) / 2;
        let mut i = i + n;
        self.tree[i] = v;
        while i > 1 {
            i >>= 1;
            self.tree[i] = transform(&self.tree[i << 1], &self.tree[(i << 1) | 1]);
        }
    }
}

pub type SegmentTreeType = $$1;
const DEFAULT_SEGMENT_TREE_TYPE: SegmentTreeType = $$2;

#[inline(always)]
pub fn transform(lhs: &SegmentTreeType, rhs: &SegmentTreeType) -> SegmentTreeType {
    $$3
}
