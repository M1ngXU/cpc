// requires Complex

// from: https://cp-algorithms.com/algebra/fft.html#improved-implementation-in-place-computation
pub fn fft(a: &mut Vec<Complex>, invert: bool) {
    let n = a.len();

    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            a.swap(i, j);
        }
    }

    let mut len = 2;
    while len <= n {
        let ang = 2. * PI / len as CT * if invert { -1. } else { 1. };
        let (s, c) = sin_cos(ang);
        let wlen = Complex(c, s);
        for i in (0..n).step_by(len) {
            let mut w = Complex::ONE;
            for j in 0..len / 2 {
                let u = a[i + j];
                let v = a[i + j + len / 2] * w;
                a[i + j] = u + v;
                a[i + j + len / 2] = u - v;
                w *= wlen;
            }
        }

        len <<= 1;
    }

    if invert {
        for x in a {
            *x /= n as CT;
        }
    }
}

pub fn multiply(a: &Vec<isize>, b: &Vec<isize>) -> Vec<isize> {
    let mut result = multiply_complex(
        &a.iter().map(|x| Complex(*x as CT, 0.)).cv(),
        &b.iter().map(|x| Complex(*x as CT, 0.)).cv(),
    )
    .into_iter()
    .map(|x| x.0.round() as I)
    .cv();
    while result.len() > 1 && result.last().unwrap() == &0 {
        result.pop().unwrap();
    }
    result
}

pub fn multiply_complex(a: &Vec<Complex>, b: &Vec<Complex>) -> Vec<Complex> {
    let mut fa = a.clone();
    let mut fb = b.clone();
    let n = (a.len() + b.len()).next_power_of_two();

    fa.resize(n, Complex::ZERO);
    fb.resize(n, Complex::ZERO);

    fft(&mut fa, false);
    fft(&mut fb, false);
    for (fai, fbi) in fa.iter_mut().zip(fb) {
        *fai *= fbi;
    }
    fft(&mut fa, true);

    fa
}
