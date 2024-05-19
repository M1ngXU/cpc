pub fn z(s: &str) -> Vec<usize> {
    let n = s.len();
    let s = s.chars().cv();
    let mut z = vec![0; n];
    let mut l = 0;
    let mut r = 0;
    for i in 1..n {
        if i < r {
            z[i] = std::cmp::min(r - i, z[i - l]);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    z
}
