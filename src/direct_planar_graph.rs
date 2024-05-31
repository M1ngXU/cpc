// edges must be Vec<HashMap<usize, cost>>
// degree must be mutable

let mut lowest_degrees = Vec::new();
for i in 0..n {
    if degree[i] <= 5 {
        lowest_degrees.push(i);
    }
}
while let Some(u) = lowest_degrees.pop() {
    for (v, _c) in edges[u].clone() {
        edges[v].remove(&u);
        degree[v] -= 1;
        if degree[v] == 5 {
            lowest_degrees.push(v);
        }
    }
}