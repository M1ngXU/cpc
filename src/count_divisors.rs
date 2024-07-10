// requires sqrt, is_prime

pub struct CountDivisors {
    primes: Vec<U>,
}
impl CountDivisors {
    pub fn new() -> Self {
        let n = 1_000_006;
        let mut lp = vec![0; n + 1];
        let mut pr = vec![];
        for i in 2..=n {
            if lp[i] == 0 {
                lp[i] = i;
                pr.push(i);
            }
            let mut j = 0;
            while i * pr[j] <= n {
                lp[i * pr[j]] = pr[j];
                if pr[j] == lp[i] {
                    break;
                }
                j += 1;
            }
        }
        Self { primes: pr }
    }
    pub fn count(&self, mut n: U) -> U {
        if n == 1 {
            return 1;
        }
        let mut ans = 1;
        for p in &self.primes {
            if p * p * p > n {
                break;
            }
            let mut count = 1;
            while n % p == 0 {
                n /= p;
                count += 1;
            }
            ans *= count;
        }
        if is_prime(n) {
            ans * 2
        } else if sqrt(n as _).is_some_and(|s| is_prime(s as U)) {
            ans * 3
        } else if n != 1 {
            ans * 4
        } else {
            ans
        }
    }
}
