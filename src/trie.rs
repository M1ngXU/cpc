#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Vertex<const K: usize> {
    next: [usize; K],
    output: bool,
}
impl<const K: usize> Vertex<K> {
    pub fn new() -> Self {
        Self {
            next: [usize::MAX; K],
            output: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Trie<const K: usize> {
    pub vertices: Vec<Vertex<K>>,
}
impl<const K: usize> Trie<K> {
    pub fn new() -> Self {
        Self {
            vertices: vec![Vertex::new()],
        }
    }

    pub fn add_string(&mut self, s: &[usize]) -> usize {
        let mut v = 0;
        for c in s {
            if self.vertices[v].next[*c] == usize::MAX {
                self.vertices[v].next[*c] = self.vertices.len();
                self.vertices[v].size += 1;
                self.vertices.push(Vertex::new());
            }
            v = self.vertices[v].next[*c];
        }
        self.vertices[v].output = true;
        v
    }

    pub fn go(&mut self, v: usize, c: usize) -> Option<usize> {
        (self.vertices[v].next[c] != usize::MAX).then_some(self.vertices[v].next[c])
    }
}
