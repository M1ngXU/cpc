fn min<T: PartialOrd>(a: T, b: T) -> T {
    match a.partial_cmp(&b) {
        Some(std::cmp::Ordering::Less) => a,
        _ => b,
    }
}

fn point_distance(a: (f64, f64, usize), b: (f64, f64, usize)) -> (f64, usize, usize) {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    (dx * dx + dy * dy, a.2, b.2)
}

fn closest_points(
    points: &Vec<(f64, f64, usize)>,
    start: usize,
    end: usize,
) -> (f64, usize, usize) {
    if end - start <= 1 {
        panic!();
    } else if end - start == 2 {
        point_distance(points[start], points[start + 1])
    } else if end - start == 3 {
        let d1 = point_distance(points[start], points[start + 1]);
        let d2 = point_distance(points[start], points[start + 2]);
        let d3 = point_distance(points[start + 1], points[start + 2]);
        min(d1, min(d2, d3))
    } else {
        let mid = (start + end) >> 1;
        let mut d = min(
            closest_points(points, start, mid),
            closest_points(points, mid, end),
        );

        let dsqrt = d.0.sqrt();

        let mut middle = points[start..end]
            .iter()
            .filter(|x| (x.0 - points[mid].0).abs() <= dsqrt)
            .copied()
            .collect::<Vec<_>>();
        middle.sort_by(|(_, y1, _), (_, y2, _)| y1.total_cmp(y2));

        for i in 0..middle.len() {
            let p1 = middle[i];
            d = middle[i + 1..]
                .iter()
                .take(7)
                .map(|p2| point_distance(p1, *p2))
                .min_by(|p2, p3| p2.partial_cmp(p3).unwrap())
                .map(|d2| min(d, d2))
                .unwrap_or(d);
        }

        d
    }
}
