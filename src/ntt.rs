// require mod_int

// to find primitive roots: https://www.wolframalpha.com/input/?i=PrimitiveRoots%5B%5B%2F%2Fnumber%3A998244353%2F%2F%5D%5D
const MOD: usize = 998244353; // 2^23 * 7 * 17 + 1
const C: usize = 119;
const ROOT_PW: usize = 1 << 23;
const G: M = M::new(3);
const ROOT: M = G.pow(C);
const ROOT_1: M = ROOT.inv();
type M = Mod<MOD>;

// from https://cp-algorithms.com/algebra/fft.html#number-theoretic-transform
pub fn fft(a: &mut Vec<M>, invert: bool) {
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
        let mut wlen = if invert { ROOT_1 } else { ROOT };
        let mut i = len;
        while i < ROOT_PW {
            wlen *= wlen;
            i <<= 1;
        }
        for i in (0..n).step_by(len) {
            let mut w = M::ONE;
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
        let n_1 = M::new(n).inv();
        for x in a {
            *x *= n_1;
        }
    }
}

pub fn multiply(a: &Vec<M>, b: &Vec<M>) -> Vec<M> {
    let mut fa = a.clone();
    let mut fb = b.clone();
    let n = (a.len() + b.len()).next_power_of_two();

    fa.resize(n, M::ZERO);
    fb.resize(n, M::ZERO);

    fft(&mut fa, false);
    fft(&mut fb, false);
    for (fai, fbi) in fa.iter_mut().zip(fb) {
        *fai *= fbi;
    }
    fft(&mut fa, true);

    fa
}
