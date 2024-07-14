fn main() {
    let _io = io();
    for _ in 0..r!(U) {
        // let (n, q) = r!(U, U);
        // let n = r!(U);
        // let a = (0..n).map(|_| r!(U)).cv();
        // START HERE
    }
}

pub fn lcm(u: U, v: U) -> U {
    u * v / gcd(u, v)
}
// gcd from wiki: https://en.wikipedia.org/wiki/Binary_GCD_algorithm
pub fn gcd(mut u: U, mut v: U) -> U {
    if u == 0 {
        return v;
    } else if v == 0 {
        return u;
    }

    let i = u.trailing_zeros();
    u >>= i;
    let j = v.trailing_zeros();
    v >>= j;
    let k = std::cmp::min(i, j);
    loop {
        if u > v {
            std::mem::swap(&mut u, &mut v);
        }
        v -= u;
        if v == 0 {
            return u << k;
        }
        v >>= v.trailing_zeros();
    }
}
pub struct Rng {
    seed: u32,
}
impl Rng {
    pub fn new() -> Self {
        Self {
            seed: std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .subsec_nanos(),
        }
    }

    pub fn next(&mut self) -> u32 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 17;
        self.seed ^= self.seed << 5;
        self.seed
    }

    pub fn next_u64(&mut self) -> u64 {
        let (l, u) = (self.next(), self.next());
        ((l as u64) << 32) | u as u64
    }

    pub fn next_u128(&mut self) -> u128 {
        let (l, u) = (self.next(), self.next());
        ((l as u128) << 64) | u as u128
    }

    pub fn next_u(&mut self) -> usize {
        self.next_u64() as usize
    }
}
pub trait IterExt<T> {
    fn n(&mut self) -> T;
    fn cv(self) -> Vec<T>;
}
impl<T, I: Iterator<Item = T>> IterExt<T> for I {
    fn cv(self) -> Vec<T> {
        self.collect()
    }

    fn n(&mut self) -> T {
        self.next().unwrap()
    }
}
pub trait IterExt2<T: PartialOrd> {
    fn mn(self) -> T;
    fn mx(self) -> T;
}

impl<T: PartialOrd, I: IntoIterator<Item = T>> IterExt2<T> for I {
    fn mn(self) -> T {
        self.into_iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }
    fn mx(self) -> T {
        self.into_iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }
}
pub trait IterExt3<T: ToString> {
    fn to_string(self, sep: &str) -> String;
}
impl<T: ToString, I: IntoIterator<Item = T>> IterExt3<T> for I {
    fn to_string(self, sep: &str) -> String {
        self.into_iter().map(|x| x.to_string()).cv().join(sep)
    }
}
pub trait IterExt4<T: Clone> {
    fn l(&self) -> T;
}
impl<T: Clone> IterExt4<T> for Vec<T> {
    fn l(&self) -> T {
        self[self.len() - 1].clone()
    }
}
impl<T: Clone> IterExt4<T> for &[T] {
    fn l(&self) -> T {
        self[self.len() - 1].clone()
    }
}
impl<T: Clone> IterExt4<T> for &mut [T] {
    fn l(&self) -> T {
        self[self.len() - 1].clone()
    }
}

pub use lib::*;
#[allow(unused)]
mod lib {
    pub type U = usize;
    pub type I = isize;
    pub type F = f64;

    pub use std::cmp::{Ordering, Reverse};
    pub use std::collections::*;
    pub use std::f64::consts::*;
    pub use std::fmt::Write;
    use std::io::*;
    use std::iter::Filter;
    use std::str::{Split, SplitWhitespace};
    pub struct Output(String);
    impl AsRef<String> for Output {
        fn as_ref(&self) -> &String {
            &self.0
        }
    }
    impl AsMut<String> for Output {
        fn as_mut(&mut self) -> &mut String {
            &mut self.0
        }
    }
    static mut OUTPUT: Output = Output(String::new());
    #[inline(always)]
    #[allow(static_mut_refs)]
    pub fn output() -> &'static mut Output {
        unsafe { &mut OUTPUT }
    }
    #[macro_export]
    macro_rules! w {
    () => {{
        let _ = writeln!(output().as_mut());
    }};
    ($t:expr) => {{
        let _ = writeln!(output().as_mut(), "{}", $t);
    }};
    ($start:expr $(,$t:expr)* $(,)?) => {{
        let o = output().as_mut();
        let _ = write!(o, "{}", $start);
        $(
            let _ = write!(o, " {}", $t);
        )*
        let _ = writeln!(o);
    }};
}
    type InputInner = (Split<'static, &'static [char]>, String);
    pub struct Input(InputInner); // must drop both at the same time (or the iterator first)
    impl Input {
        #[inline(always)]
        pub fn get_next(&mut self) -> &'static str {
            loop {
                if let Some(n) = self.0 .0.next() {
                    if !n.is_empty() {
                        return n;
                    }
                } else {
                    self.0 = read_next_line();
                }
            }
        }
    }
    const SKIP_CHARS: [char; 5] = [' ', '\n', '\r', '\t', ','];
    fn read_next_line() -> InputInner {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        let ss: &'static mut str = unsafe { std::mem::transmute(s.as_mut_str()) };
        (ss.split(&SKIP_CHARS), s)
    }
    static mut INPUT: Option<Input> = None;
    #[inline(always)]
    #[allow(static_mut_refs)]
    pub fn input() -> &'static mut Input {
        unsafe { INPUT.as_mut().unwrap_unchecked() }
    }
    pub trait Parse {
        type ParsesTo;

        #[inline(always)]
        fn parse(i: &mut Input) -> Self::ParsesTo;
    }
    pub struct IO;
    impl Drop for IO {
        fn drop(&mut self) {
            print!("{}", output().0);
        }
    }
    macro_rules! impl_parse {
    ($($t:tt),*) => {
        $(
        impl Parse for $t {
            type ParsesTo = Self;

            #[inline(always)]
            fn parse(i: &mut Input) -> Self::ParsesTo {
                <Self as std::str::FromStr>::from_str(&i.get_next()).unwrap()
            }
        }
        )*
    };
}

    impl_parse!(
        bool, u8, i8, char, u16, i16, u32, i32, f32, u64, i64, f64, usize, isize, u128, i128
    );
    impl Parse for String {
        type ParsesTo = Vec<char>;

        fn parse(i: &mut Input) -> Self::ParsesTo {
            i.get_next().chars().collect()
        }
    }
    impl<T: Parse<ParsesTo = U>, U> Parse for Vec<T> {
        type ParsesTo = Vec<U>;

        fn parse(i: &mut Input) -> Self::ParsesTo {
            let n = usize::parse(i);
            (0..n).map(|_| T::parse(i)).collect()
        }
    }
    #[inline(always)]
    pub fn lin<T: Parse<ParsesTo = U>, U, V: FromIterator<U>>(n: usize) -> V {
        let i = input();
        (0..n).map(|_| T::parse(i)).collect()
    }
    #[inline(always)]
    pub fn grid<T: Parse<ParsesTo = U>, U, V: FromIterator<U>, W: FromIterator<V>>(
        n: usize,
        m: usize,
    ) -> W {
        (0..n).map(|_| lin::<T, U, V>(m)).collect()
    }
    #[macro_export]
    macro_rules! rv {
        ($n:expr; $c:tt<$t:tt>) => {
            lin::<$t, _, $c<_>>($n)
        };
        ($n:expr, $m:expr; $c1:tt<$c2:tt<$t:tt>>) => {
            grid::<$t, _, $c2<_>, $c1<$c2<_>>>($n, $m);
        };
    }
    #[macro_export]
    macro_rules! r {
    ($($t:ty),* $(,)?) => {{
        let i = input();
        ($(<$t as Parse>::parse(i)),*)
    }};
}
    pub fn io() -> IO {
        let (mut i, mut o) = (Some(Input(read_next_line())), Output(String::new()));
        unsafe {
            INPUT = i;
            OUTPUT = o;
        }
        IO
    }
}
