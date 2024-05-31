// requires fft, complex

/// `a`, `b` lowercase latin character or wildcard
pub fn wildcard_match(a: &str, b: &str, wildcard: char) -> Vec<bool> {
    let ap = a
        .chars()
        .map(|c| {
            if c == wildcard {
                None
            } else {
                Some(c as usize - b'a' as usize)
            }
        })
        .map(|x| x.map(|x| 2. * PI / 26. * x as F))
        .map(|ang| {
            ang.map(|ang| Complex(ang.cos(), ang.sin()))
                .unwrap_or(Complex::ZERO)
        })
        .cv();
    let bp = b
        .chars()
        .map(|c| {
            if c == wildcard {
                None
            } else {
                Some(c as usize - b'a' as usize)
            }
        })
        .map(|x| x.map(|x| 2. * PI / 26. * x as F))
        .map(|ang| {
            ang.map(|ang| Complex(ang.cos(), -ang.sin()))
                .unwrap_or(Complex::ZERO)
        })
        .rev()
        .cv();
    let duplicates = multiply(
        &a.chars().map(|c| if c == wildcard { 1 } else { 0 }).cv(),
        &b.chars()
            .map(|c| if c == wildcard { 1 } else { 0 })
            .rev()
            .cv(),
    );
    let result = multiply_complex(&ap, &bp).into_iter().map(|x| x.0).cv();
    let mut prefix_wildcard = vec![0];
    for c in a.chars() {
        prefix_wildcard.push(prefix_wildcard.l() + if c == wildcard { 1 } else { 0 });
    }
    let b_wildcard = b.chars().filter(|c| c == &wildcard).count();
    let mut out = Vec::new();
    for i in b.len() - 1..a.len() {
        let wildcards = b_wildcard + prefix_wildcard[i + 1] - prefix_wildcard[i - (b.len() - 1)];
        out.push(
            ((result[i] + wildcards as F - *duplicates.get(i).unwrap_or(&0) as F) - b.len() as F)
                .abs()
                < 1e-6,
        );
    }
    out
}
