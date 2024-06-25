#[inline]
fn orient2d(pa: [f64; 2], pb: [f64; 2], pc: [f64; 2]) -> f64 {
    let detleft = (pa[0] - pc[0]) * (pb[1] - pc[1]);
    let detright = (pa[1] - pc[1]) * (pb[0] - pc[0]);
    let det = detleft - detright;
    return det;
}

type Point = [f64; 2];

fn circumcircle(a: Point, b: Point, c: Point) -> (Point, f64) {
    let orientation = orient2d(a, b, c);

    let (b, c, denominator) = if orientation > 0. {
        (b, c, 2. * orientation)
    } else if orientation < 0. {
        (c, b, -2. * orientation)
    } else {
        panic!()
    };

    let [acx, acy, bcx, bcy, abx, aby] = [
        a[0] - c[0],
        a[1] - c[1],
        b[0] - c[0],
        b[1] - c[1],
        a[0] - b[0],
        a[1] - b[1],
    ];
    let [acxs, acys, bcxs, bcys, abxs, abys] = [
        acx * acx,
        acy * acy,
        bcx * bcx,
        bcy * bcy,
        abx * abx,
        aby * aby,
    ];
    let [acxys, bcxys, abxys] = [acxs + acys, bcxs + bcys, abxs + abys];
    let center = [
        c[0] + (acxys * bcy - bcxys * acy) / denominator,
        c[1] + (acx * bcxys - bcx * acxys) / denominator,
    ];
    let radius = f64::sqrt(bcxys * acxys * abxys) / denominator;
    (center, radius)
}
