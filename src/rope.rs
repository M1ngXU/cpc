pub struct Rope {
    pub tree: Vec<usize>,
}
impl Rope {
    pub fn new(input: &[usize]) -> Self {
        let n = input.len().next_power_of_two();
        let mut tree = Vec::with_capacity(2 * n - 1);
        unsafe {
            tree.set_len(2 * n - 1);
        }
        tree[n - 1..n - 1 + input.len()].clone_from_slice(input);
        for i in n - 1 + input.len()..2 * n - 1 {
            tree[i] = 0;
        }
        for i in (0..n - 1).rev() {
            tree[i] = tree[2 * i + 1] + tree[2 * i + 2];
        }
        Self { tree }
    }

    /// Query the segment tree for the range [l, r).
    pub fn query(&self, v: usize) -> Option<usize> {
        self.query_rec(0, 0, (self.tree.len() + 1) >> 1, v)
    }

    fn query_rec(&self, i: usize, il: usize, ir: usize, v: usize) -> Option<usize> {
        if v == 1 && il + 1 == ir {
            return Some(i - ((self.tree.len() + 1) / 2 - 1));
        }
        let m = (il + ir) >> 1;
        if self.tree[2 * i + 1] < v {
            self.query_rec(2 * i + 2, m, ir, v - self.tree[2 * i + 1])
        } else {
            self.query_rec(2 * i + 1, il, m, v)
        }
    }

    pub fn update(&mut self, i: usize) {
        let n = (self.tree.len() + 1) / 2;
        let mut i = i + n - 1;
        self.tree[i] = 0;
        while i > 0 {
            i = (i - 1) / 2;
            self.tree[i] = self.tree[2 * i + 1] + self.tree[2 * i + 2];
        }
    }
}
