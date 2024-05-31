pub fn polyprod(p: Vec<Vec<isize>>, m: isize) -> Vec<isize> {
    let mut polynomials = std::collections::BinaryHeap::new();
    for p in p {
        polynomials.push(Comparator(p));
    }
    while polynomials.len() > 1 {
        let x = Comparator(
            multiply(&polynomials.pop().unwrap().0, &polynomials.pop().unwrap().0)
                .into_iter()
                .map(|x| x % m)
                .cv(),
        );
        polynomials.push(x);
    }
    polynomials.pop().unwrap().0
}

#[derive(Debug)]
struct Comparator(Vec<I>);
impl PartialEq for Comparator {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}
impl Eq for Comparator {}
impl Ord for Comparator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.len().cmp(&other.0.len()).reverse()
    }
}
impl PartialOrd for Comparator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
