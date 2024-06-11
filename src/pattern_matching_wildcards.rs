// requires ntt, modint

/// `a`, `b` lowercase latin character or wildcard
/// `a`, `b` lowercase latin character or wildcard
pub fn wildcard_match(a: &[char], b: &[char], wildcard: char) -> Vec<bool> {
    let mut rng = Rng::new();
    let enc = (0..26)
        .map(|_| M::new(rng.next() as U))
        .map(|x| if x.0 < 5 { x + 5 } else { x })
        .map(|x| (x, M::ONE / x))
        .cv();
    let ap = a
        .iter()
        .copied()
        .map(|c| {
            if c == wildcard {
                M::ZERO
            } else {
                enc[c as usize - b'a' as usize].0
            }
        })
        .cv();
    let bp = b
        .iter()
        .copied()
        .map(|c| {
            if c == wildcard {
                M::ZERO
            } else {
                enc[c as usize - b'a' as usize].1
            }
        })
        .rev()
        .cv();
    let duplicates = multiply(
        &a.iter()
            .copied()
            .map(|c| if c == wildcard { M::ONE } else { M::ZERO })
            .cv(),
        &b.iter()
            .copied()
            .map(|c| if c == wildcard { M::ONE } else { M::ZERO })
            .rev()
            .cv(),
    );
    let result = multiply(&ap, &bp).into_iter().map(|x| x.0).cv();
    let mut prefix_wildcard = vec![0];
    for c in a.iter().copied() {
        prefix_wildcard.push(prefix_wildcard.l() + if c == wildcard { 1 } else { 0 });
    }
    let b_wildcard = b.iter().copied().filter(|c| c == &wildcard).count();
    let mut out = Vec::new();
    for i in b.len() - 1..a.len() {
        let wildcards = b_wildcard + (prefix_wildcard[i + 1] - prefix_wildcard[i - (b.len() - 1)]);
        out.push(
            result[i] as I
                == b.len() as I + duplicates.get(i).map(|x| x.0).unwrap_or(0) as I - wildcards as I,
        );
    }
    out
}
