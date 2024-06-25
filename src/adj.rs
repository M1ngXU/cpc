let adj = |i, j| {
    [(-1, 0), (0, 1), (0, -1), (1, 0)]
        .into_iter()
        .map(move |(x, y)| (i as isize + x, j as isize + y))
        .filter(|&(i, j)| 0 <= i && i < n as isize && 0 <= j && j < m as isize)
        .map(|(i, j)| (i as usize, j as usize))
};