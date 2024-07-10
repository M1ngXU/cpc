// for minimal weighted matching simply invert the edge costs,
// but note that an empty matching (e.g. if all edges negative) has a weight of `0`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaximumWeightBipartiteMatching {
    graph: Vec<Vec<(usize, isize)>>,
    alen: usize,
}
impl MaximumWeightBipartiteMatching {
    pub fn new(mut a: Vec<Vec<(usize, isize)>>, blen: usize) -> Self {
        for v in &mut a {
            v.sort_unstable();
        }
        let alen = a.len();
        let mut b = vec![vec![]; blen];
        for (u, connected_to) in a.iter_mut().enumerate() {
            for (v, c) in connected_to.iter_mut() {
                b[*v].push((u + 1, isize::MAX));
                *v += alen + 1;
                *c = c.saturating_neg();
            }
            connected_to.insert(0, (0, isize::MAX));
        }
        for u in 0..blen {
            b[u].push((alen + blen + 1, 0));
        }
        a.insert(0, (1..=alen).map(|x| (x, 0)).cv());
        a.extend(b);
        a.push((alen + 1..alen + 1 + blen).map(|x| (x, isize::MAX)).cv());
        Self { graph: a, alen }
    }

    pub fn calculate(mut self) -> Vec<(usize, usize, isize)> {
        let mut matching = HashSet::new();
        while let Some(augment) = self.bellman_ford() {
            for i in 1..augment.len() {
                let (u, v) = (augment[i - 1], augment[i]);
                let vi = self.graph[u].binary_search_by_key(&v, |x| x.0).unwrap();
                let ui = self.graph[v].binary_search_by_key(&u, |x| x.0).unwrap();
                self.graph[v][ui].1 = -self.graph[u][vi].1;
                self.graph[u][vi].1 = isize::MAX;
                if u < v {
                    assert!(matching.insert((u, v, -self.graph[v][ui].1)));
                } else {
                    assert!(matching.remove(&(v, u, self.graph[v][ui].1)));
                }
            }
        }
        matching
            .into_iter()
            .filter(|&(u, v, _)| u != 0 && v != self.graph.len() - 1)
            .map(|(u, v, c)| (u - 1, v - 1 - self.alen, c.saturating_neg()))
            .cv()
    }

    fn bellman_ford(&self) -> Option<Vec<usize>> {
        let mut dist = vec![isize::MAX; self.graph.len()];
        dist[0] = 0;
        let mut parent = vec![usize::MAX; self.graph.len()];

        for j in 0..dist.len() {
            let mut changed = false;
            for u in 0..self.graph.len() {
                if dist[u] == isize::MAX {
                    continue;
                }
                for (v, c) in &self.graph[u] {
                    if *c != isize::MAX {
                        if dist[u] + c < dist[*v] {
                            dist[*v] = dist[u] + c;
                            parent[*v] = u;
                            changed = true;
                        }
                    }
                }
            }
            if !changed {
                break;
            }
            if j + 1 == dist.len() {
                panic!("Negative cycle!");
            }
        }
        if dist.l() >= 0 {
            return None;
        }
        let mut cur = dist.len() - 1;
        let mut path = vec![cur];
        while parent[cur] != usize::MAX {
            path.push(parent[cur]);
            cur = parent[cur];
        }
        path.reverse();
        Some(path)
    }
}
