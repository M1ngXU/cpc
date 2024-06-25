fn pow(a: usize, mut e: usize, n: usize) -> usize {
    let mut res = 1;
    let mut cur = a;
    while e > 0 {
        if e & 1 == 1 {
            res = ((res as u128 * cur as u128) % n as u128) as usize;
        }
        cur = ((cur as u128 * cur as u128) % n as u128) as usize;
        e >>= 1;
    }
    res
}

fn is_prime(n: U) -> bool {
    if n == 1 {
        false
    } else if n == 2 {
        true
    } else if n == 3 {
        true
    } else if n & 1 == 0 {
        false
    } else {
        let mut rng = Rng::new();
        let s = (n - 1).trailing_zeros();
        let d = (n - 1) >> s;
        for _ in 0..100 {
            let a = rng.next() as U % (n - 4) + 2;
            let mut x = pow(a, d, n);
            for _ in 0..s {
                let y = ((x as u128 * x as u128) % n as u128) as usize;
                if y == 1 && x != 1 && x != n - 1 {
                    return false;
                }
                x = y;
            }
            if x != 1 {
                return false;
            }
        }
        true
    }
}
