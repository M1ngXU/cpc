#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Vertex<const K: usize> {
    // can be changed to "go" if first all strings are added
    next: [usize; K],
    output: bool,
    parent: usize,
    parent_char: usize,
    link: usize,
    go: [usize; K],
    depth: usize,
}
impl<const K: usize> Vertex<K> {
    pub fn new(parent: usize, parent_char: usize, depth: usize) -> Self {
        Self {
            go: [usize::MAX; K],
            link: usize::MAX,
            next: [usize::MAX; K],
            output: false,
            depth,
            parent,
            parent_char,
        }
    }
}

pub type LAhoCorasick = AhoCorasick<26>;
#[derive(Clone, Debug)]
pub struct AhoCorasick<const K: usize> {
    pub vertices: Vec<Vertex<K>>,
}
impl<const K: usize> AhoCorasick<K> {
    pub fn new() -> Self {
        Self {
            vertices: vec![Vertex::new(usize::MAX, usize::MAX, 0)],
        }
    }

    pub fn add_string(&mut self, s: &[usize]) -> usize {
        let mut v = 0;
        for c in s {
            if self.vertices[v].next[*c] == usize::MAX {
                self.vertices[v].next[*c] = self.vertices.len();
                self.vertices
                    .push(Vertex::new(v, *c, self.vertices[v].depth + 1));
            }
            v = self.vertices[v].next[*c];
        }
        self.vertices[v].output = true;
        v
    }

    pub fn go(&mut self, v: usize, c: usize) -> usize {
        if self.vertices[v].go[c] == usize::MAX {
            if self.vertices[v].next[c] != usize::MAX {
                self.vertices[v].go[c] = self.vertices[v].next[c];
            } else {
                self.vertices[v].go[c] = if v == 0 {
                    0
                } else {
                    let l = self.get_link(v);
                    self.go(l, c)
                };
            }
        }
        self.vertices[v].go[c]
    }

    fn get_link(&mut self, v: usize) -> usize {
        if self.vertices[v].link == usize::MAX {
            if v == 0 || self.vertices[v].parent == 0 {
                self.vertices[v].link = 0;
            } else {
                let l = self.get_link(self.vertices[v].parent);
                self.vertices[v].link = self.go(l, self.vertices[v].parent_char);
            }
        }
        self.vertices[v].link
    }
}
