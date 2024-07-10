pub struct FenwickTree {
    tree: Vec<FenwickTreeType>,
}
impl FenwickTree {
    pub fn new(n: usize) -> Self {
        Self {
            tree: vec![DEFAULT; n],
        }
    }

    pub fn query(&self, mut i: usize) -> FenwickTreeType {
        i += 1;
        let mut sum = 0;
        while i > 0 {
            sum += self.tree[i - 1];
            i -= ((i as isize) & -(i as isize)) as usize;
        }
        sum
    }

    pub fn update(&mut self, mut i: usize, v: FenwickTreeType) {
        i += 1;
        while i <= self.tree.len() {
            self.tree[i - 1] += v;
            i += ((i as isize) & -(i as isize)) as usize;
        }
    }
}
type FenwickTreeType = I;
const DEFAULT: FenwickTreeType = FenwickTreeType::default();
