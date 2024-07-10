#[derive(Debug)]
struct Comparator(compile_error!());
impl PartialEq for Comparator {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}
impl Eq for Comparator {}
impl Ord for Comparator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!()
    }
}
impl PartialOrd for Comparator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
