pub fn dfs(u: U, edges: &Vec<Vec<U>>, dfsin: &mut Vec<U>, dfsout: &mut Vec<U>, t: &mut U) {
    let mut todo = vec![(u, vec![])];
    while let Some((u, mut vv)) = todo.pop() {
        dfsin[u] = *t;
        *t += 1;

        if edges[u].is_empty() {
            dfsout[u] = *t;
            *t += 1;
            for v in vv.into_iter().rev() {
                dfsout[v] = *t;
                *t += 1;
            }
        } else {
            vv.push(u);
            let mut todo2 = vec![];
            for v in &edges[u] {
                if *v == edges[u].l() {
                    todo2.push((*v, vv));
                    break;
                } else {
                    todo2.push((*v, vec![]));
                }
            }
            todo2.reverse();
            todo.extend(todo2);
        }
    }
}
