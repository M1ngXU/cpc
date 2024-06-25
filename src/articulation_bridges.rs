let mut dfsin = vec![0; n];
let mut low = vec![0; n];
let mut subtree_size = vec![1; n];
let mut is_art = vec![false; n];
fn dfs(
    u: U,
    edges: &Vec<Vec<U>>,
    dfsin: &mut Vec<U>,
    low: &mut Vec<U>,
    subtree_size: &mut Vec<U>,
    is_art: &mut Vec<bool>,
    tt: &mut HashSet<(U, U)>,
    bridges: &mut Vec<(U, U)>,
    t: &mut U,
) -> U {
    *t += 1;
    dfsin[u] = *t;
    low[u] = *t;
    for v in &edges[u] {
        if dfsin[*v] == 0 {
            tt.insert((*v.min(&u), *v.max(&u)));
            let val = dfs(*v, edges, dfsin, low, subtree_size, is_art, tt, bridges, t);
            subtree_size[u] += subtree_size[*v];
            if val >= dfsin[u] {
                is_art[u] = true;
            }
            low[u] = low[u].min(val);
        } else if dfsin[*v] != 0 && !tt.contains(&(*v.min(&u), *v.max(&u))) {
            low[u] = low[u].min(dfsin[*v]);
        }
    }
    low[u]
}
let mut tt = HashSet::new();
let mut bridges = vec![];
dfs(
    0,
    &graph,
    &mut dfsin,
    &mut low,
    &mut subtree_size,
    &mut is_art,
    &mut tt,
    &mut bridges,
    &mut 0,
);
is_art[0] = tt.into_iter().filter(|x| x.0 == 0).count() >= 2;
for (u, v) in edges {
    if low[u] > dfsin[v] || low[v] > dfsin[u] {
        bridges.push((u, v));
    }
}