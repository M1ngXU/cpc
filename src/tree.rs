let mut edges = vec![vec![]; n];
for _ in 1..n {
    let (u, v) = r!(usize, usize);
    edges[u - 1].push(v - 1);
    edges[v - 1].push(u - 1);
}