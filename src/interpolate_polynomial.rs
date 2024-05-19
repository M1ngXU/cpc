fn mul_add_row(mut row_a: [N; 11], row_b: [N; 11], f: N) -> [N; 11] {
    for i in 0..11 {
        row_a[i] += row_b[i] * f;
    }
    row_a
}

fn solve(mut matrix: [[N; 11]; 11], mut b: [N; 11]) -> [N; 11] {
    for i in 0..11 {
        let f = matrix[i][i];
        for j in 0..11 {
            matrix[i][j] /= f;
        }
        b[i] /= f;
        for j in i + 1..11 {
            b[j] -= b[i] * matrix[j][i];
            matrix[j] = mul_add_row(matrix[j], matrix[i], -matrix[j][i]);
        }
    }
    for i in (1..11).rev() {
        for j in 0..i {
            b[j] -= b[i] * matrix[j][i];
            matrix[j] = mul_add_row(matrix[j], matrix[i], -matrix[j][i]);
        }
    }
    b
}

fn eval(x: N, coef: &[N; 11]) -> N {
    let mut res = N::ZERO;
    for (i, c) in coef.iter().enumerate() {
        res += x.pow(i) * *c;
    }
    res
}
