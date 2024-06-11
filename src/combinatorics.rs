static mut SEEN: Vec<(N, N)> = Vec::new();

#[allow(static_mut_refs)]
fn get_seen() -> &'static mut Vec<(N, N)> {
    unsafe { &mut SEEN }
}

/// use `init_seen` to get initial "dp" memoization table, doubles size on miss (why not)
pub trait Combinatorics {
    fn choose(self, k: usize) -> N;
    fn fact(self) -> N;
    fn inv_fact(self) -> N;
}
impl Combinatorics for usize {
    fn choose(self, k: usize) -> N {
        if self < k {
            N::ZERO
        } else {
            self.fact() * (self - k).inv_fact() * k.inv_fact()
        }
    }
    fn fact(self) -> N {
        let seen = get_seen();
        while self >= seen.len() {
            init_seen(seen);
        }
        seen[self].0
    }
    fn inv_fact(self) -> N {
        let seen = get_seen();
        while self >= seen.len() {
            init_seen(seen);
        }
        seen[self].1
    }
}
fn init_seen(seen: &mut Vec<(N, N)>) {
    if seen.is_empty() {
        seen.push((N::ONE, N::ONE));
    }
    let mut cur = seen.l().0;
    let mut cur_inv = seen.l().1;
    for i in seen.len()..seen.len() * 2 {
        cur *= N::new(i);
        cur_inv /= N::new(i);
        seen.push((cur, cur_inv));
    }
}
