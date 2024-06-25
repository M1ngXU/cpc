use std::cmp::Ordering;

/// Tries to calculate the square root of x, `None` if `x` is not a perfect square
pub fn sqrt(x: u128) -> Option<u128> {
    let mut l = 0;
    let mut r = x + 1;
    while l <= r {
        let mid = (l + r) >> 1;
        match (mid.saturating_mul(mid)).cmp(&x) {
            Ordering::Equal => return Some(mid),
            Ordering::Greater => r = mid - 1,
            Ordering::Less => l = mid + 1,
        }
    }
    None
}
