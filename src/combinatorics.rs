/// use `init_seen` to get initial "dp" memoization table, doubles size on miss (why not)
pub trait Combinatorics {
    fn choose(self, k: usize, seen: &mut Vec<(N, N)>) -> N;
    fn fact(self, seen: &mut Vec<(N, N)>) -> N;
    fn inv_fact(self, seen: &mut Vec<(N, N)>) -> N;
}
impl Combinatorics for usize {
    fn choose(self, k: usize, seen: &mut Vec<(N, N)>) -> N {
        self.fact(seen) * (self - k).inv_fact(seen) * k.inv_fact(seen)
    }
    fn fact(self, seen: &mut Vec<(N, N)>) -> N {
        while self >= seen.len() {
            init_seen(seen);
        }
        seen[self].0
    }
    fn inv_fact(self, seen: &mut Vec<(N, N)>) -> N {
        while self >= seen.len() {
            init_seen(seen);
        }
        seen[self].1
    }
}
pub fn get_seen() -> Vec<(N, N)> {
    vec![(N::ONE, N::ONE)]
}
fn init_seen(seen: &mut Vec<(N, N)>) {
    let mut cur = seen.l().0;
    let mut cur_inv = seen.l().1;
    for i in seen.len()..seen.len() * 2 {
        cur *= N::new(i);
        cur_inv /= N::new(i);
        seen.push((cur, cur_inv));
    }
}
