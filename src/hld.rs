use std::mem::swap;

pub struct Hld {
    heavy: Vec<usize>,
    in_num: Vec<usize>,
    out_num: Vec<usize>,
    light: Vec<usize>,
    depth: Vec<usize>,
}
impl Hld {
    fn max_subtree(
        u: usize,
        graph: &mut Vec<Vec<usize>>,
        subtree_sizes: &mut Vec<usize>,
        max: &mut Vec<usize>,
        light: &mut Vec<usize>,
        depth: &mut Vec<usize>,
    ) {
        subtree_sizes[u] = 0;
        for i in (0..graph[u].len()).rev() {
            let v = graph[u][i];
            if depth[v] != usize::MAX {
                if max[u] == graph[u].len() - 1 {
                    max[u] = i;
                }
                graph[u].swap_remove(i);
                continue;
            }
            light[v] = u;
            depth[v] = depth[u] + 1;
            Self::max_subtree(v, graph, subtree_sizes, max, light, depth);
            subtree_sizes[u] += subtree_sizes[v];
            if subtree_sizes[graph[u][max[u]]] < subtree_sizes[v] {
                max[u] = i;
            }
        }
    }

    fn hld(
        u: usize,
        graph: &Vec<Vec<usize>>,
        max: &Vec<usize>,
        in_num: &mut Vec<usize>,
        out_num: &mut Vec<usize>,
        heavy: &mut Vec<usize>,
        ctr: &mut usize,
    ) {
        in_num[u] = *ctr;
        *ctr += 1;

        if !graph[u].is_empty() {
            heavy[graph[u][max[u]]] = heavy[u];
            Self::hld(graph[u][max[u]], graph, max, in_num, out_num, heavy, ctr);
        }
        for (i, v) in graph[u].iter().enumerate() {
            if max[u] == i {
                continue;
            };
            heavy[*v] = *v;
            Self::hld(*v, graph, max, in_num, out_num, heavy, ctr);
        }

        out_num[u] = *ctr;
    }

    pub fn new(mut graph: Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        let mut subtree_sizes = vec![0; n];
        let mut max = vec![0; n];
        let mut light = vec![0; n];
        let mut depth = vec![usize::MAX; n];
        depth[0] = 0;
        Self::max_subtree(
            0,
            &mut graph,
            &mut subtree_sizes,
            &mut max,
            &mut light,
            &mut depth,
        );
        let mut in_num = subtree_sizes; // reuse :)
        let mut out_num = vec![0; n];
        let mut heavy = vec![0; n];
        Self::hld(
            0,
            &graph,
            &max,
            &mut in_num,
            &mut out_num,
            &mut heavy,
            &mut 0,
        );

        Self {
            in_num,
            out_num,
            heavy,
            light,
            depth,
        }
    }

    fn is_ancestor(&self, a: usize, b: usize) -> bool {
        self.in_num[a] <= self.in_num[b] && self.out_num[b] <= self.out_num[a]
    }

    pub fn query(&self, mut a: usize, mut b: usize) -> Vec<(usize, usize)> {
        let mut segments = vec![];
        while a != b {
            if self.is_ancestor(b, a) {
                swap(&mut a, &mut b);
            }
            if self.is_ancestor(a, b) && self.is_ancestor(self.heavy[b], a) {
                // a is on the heavy path from b
                segments.push((self.in_num[a], self.in_num[b]));
                b = a;
                continue;
            }
            if self.depth[self.heavy[a]] < self.depth[self.heavy[b]] {
                swap(&mut a, &mut b);
            }
            segments.push((self.in_num[self.heavy[a]], self.in_num[a]));
            a = self.light[self.heavy[a]];
        }
        if !segments
            .last()
            .is_some_and(|x| x.0 == self.in_num[a] || x.1 == self.in_num[a])
        {
            segments.push((self.in_num[a], self.in_num[a]));
        }
        segments
    }

    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        while a != b {
            if self.is_ancestor(b, a) {
                swap(&mut a, &mut b);
            }
            if self.is_ancestor(a, b) && self.is_ancestor(self.heavy[b], a) {
                // a is on the heavy path from b
                b = a;
                continue;
            }
            if self.depth[self.heavy[a]] < self.depth[self.heavy[b]] {
                swap(&mut a, &mut b);
            }
            a = self.light[self.heavy[a]];
        }
        a
    }
}
