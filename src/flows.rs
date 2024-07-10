use std::cmp::Reverse;
use std::collections::BinaryHeap;
/// Ford-Fulkerson flow/min-cut calculation, can modify (only increase?) capacities after max flow
/// calculation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FFFlows {
    network: Vec<Vec<(usize, usize)>>,
    rev: Vec<Vec<(usize, usize)>>,
    indeces: Vec<HashMap<usize, usize>>,
    flow: Vec<Vec<usize>>,
}
impl FFFlows {
    pub fn new(network: Vec<Vec<(usize, usize)>>) -> Self {
        let flow = network.iter().map(|e| vec![0; e.len()]).cv();
        let mut rev = vec![vec![]; network.len()];
        let mut indeces = vec![HashMap::new(); network.len()];
        for (u, e) in network.iter().enumerate() {
            for (i, (v, _)) in e.iter().enumerate() {
                indeces[u].insert(*v, i);
                rev[*v].push((u, i));
            }
        }
        Self {
            flow,
            indeces,
            rev,
            network,
        }
    }

    /// Calculate the maximum flow between source/sink, use `max_flow != 0` if some capacities were
    /// increased
    pub fn calculate(
        &mut self,
        mut max_flow: usize,
        source: usize,
        sink: usize,
    ) -> (usize, Vec<bool>) {
        loop {
            match self.find_augmenting_path(source, sink) {
                Ok(augmenting_path) => {
                    max_flow += augmenting_path.get(0).map(|x| x.3).unwrap_or(0).abs() as usize;
                    for (_, u, i, increase) in augmenting_path {
                        if increase > 0 {
                            self.flow[u][i] += increase as usize;
                        } else {
                            self.flow[u][i] -= (-increase) as usize;
                        }
                    }
                }
                Err(cut) => {
                    return (max_flow, cut);
                }
            }
        }
    }

    fn find_augmenting_path(
        &self,
        source: usize,
        sink: usize,
    ) -> Result<Vec<(usize, usize, usize, isize)>, Vec<bool>> {
        let mut parent = vec![(usize::MAX, usize::MAX, usize::MAX, 0_isize); self.network.len()];
        let mut dists = vec![usize::MAX; self.network.len()];
        parent[source].0 = source;
        dists[source] = 0;
        let mut todo = BinaryHeap::new();
        todo.push((Reverse(0), source));
        while let Some((Reverse(d), u)) = todo.pop() {
            if dists[u] < d {
                continue;
            }
            if u == sink {
                let mut path = Vec::new();
                let mut cur = parent[sink];
                while cur.1 != usize::MAX {
                    path.push(cur);
                    cur = parent[cur.0];
                }
                path.reverse();
                let eps = path.iter().map(|x| x.3.abs()).mn();
                eprintln!("{eps}");
                for x in &mut path {
                    x.3 = x.3.clamp(-eps, eps);
                }
                return Ok(path);
            }
            for (i, ((v, capacity), flow)) in self.network[u].iter().zip(&self.flow[u]).enumerate()
            {
                if flow < capacity {
                    let diff = capacity - flow;
                    if dists[*v] > d + diff {
                        dists[*v] = d + diff;
                        parent[*v] = (u, u, i, diff as isize);
                        todo.push((Reverse(d + diff), *v));
                    }
                }
            }
            for (v, i) in &self.rev[u] {
                let diff = self.flow[*v][*i];
                if diff > 0 {
                    if dists[*v] > d + diff {
                        dists[*v] = d + diff;
                        parent[*v] = (u, *v, *i, -(diff as isize));
                        todo.push((Reverse(d + diff), *v));
                    }
                }
            }
        }
        Err(parent.into_iter().map(|(p, _, _, _)| p != usize::MAX).cv())
    }

    pub fn increase_capacity(&mut self, u: usize, v: usize, increase_by: usize) {
        self.network[u][self.indeces[u][&v]].1 += increase_by;
    }

    pub fn get_capacity(&self, u: usize, v: usize) -> Option<usize> {
        self.indeces[u].get(&v).map(|&i| self.network[u][i].1)
    }

    pub fn get_flow(&self, u: usize, v: usize) -> Option<usize> {
        self.indeces[u].get(&v).map(|&i| self.flow[u][i])
    }
}

