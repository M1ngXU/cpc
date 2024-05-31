fn main() {
    let _io = Io::new();
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

// from https://github.com/TecTrixer/cp-template/blob/main/temp.rs
use lib::*;
#[rustfmt::skip]
mod lib {
#![allow(dead_code, unused_imports)]
pub use std::collections::{HashMap, HashSet};
use std::{
fmt::Display,
io::{
    stdin, stdout, BufRead, BufReader, BufWriter, Cursor, Error, ErrorKind, Read, Stdin,
    Stdout, Write,
},
str::{from_utf8_unchecked, FromStr}, ops::Deref,
};
pub static mut IO: Option<IoInner<Stdin, Stdout>> = None;
pub type U = usize;
pub type I = isize;
pub type F = f64;
pub type B = u8;
fn is_skip_char(&b: &u8) -> bool {
    b == b' ' || b == b'\n' || b == b'\r' || b == b'\t' || b == b','
}
pub struct IoInner<R, W>
where
    R: Read,
    W: Write,
{
    input: BufReader<R>,
    output: BufWriter<W>,
}
pub struct Io;
impl std::ops::DerefMut for Io {
    fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { IO.as_mut().unwrap_unchecked() }
    }
}
impl Deref for Io {
    type Target = IoInner<Stdin, Stdout>;
    fn deref(&self) -> &Self::Target {
            unsafe { IO.as_mut().unwrap_unchecked() }
    }
}
impl Io {
pub fn new() -> Self {
    unsafe {
        IO = Some(IoInner::new());
    }
    Self
}
}
impl IoInner<&[u8], Stdout> {
#[allow(clippy::should_implement_trait)]
pub fn from_str(input: &str) -> IoInner<&[u8], Stdout> {
    IoInner {
        input: BufReader::new(input.as_bytes()),
        output: BufWriter::new(stdout()),
    }
}
pub fn from_string(input: String) -> IoInner<Cursor<String>, Stdout> {
    IoInner {
        input: BufReader::new(Cursor::new(input)),
        output: BufWriter::new(stdout()),
    }
}
}
impl IoInner<Stdin, Stdout> {
pub fn new() -> IoInner<Stdin, Stdout> {
    IoInner {
        input: BufReader::new(stdin()),
        output: BufWriter::new(stdout()),
    }
}
}
impl Drop for Io {
fn drop(&mut self) {
    self.flush();
}
}
impl<R: Read, W: Write> IoInner<R, W> {
pub fn with_reader_and_writer(reader: R, writer: W) -> IoInner<R, W> {
    IoInner {
        input: BufReader::new(reader),
        output: BufWriter::new(writer),
    }
}
pub fn r<T: FromStr>(&mut self) -> T {
    let buf = self
        .input
        .by_ref()
        .bytes()
        .map(|x| unsafe { x.unwrap_unchecked() })
        .skip_while(is_skip_char)
        .take_while(|c| !is_skip_char(c))
        .collect::<Vec<_>>();
    unsafe { from_utf8_unchecked(&buf) }
        .parse()
        .map_err(|_| Error::new(ErrorKind::Other, "could not parse value"))
        .unwrap()
}
pub fn read_line(&mut self) -> String {
    let mut res = String::new();
    unsafe {
        self.input.read_line(&mut res).unwrap_unchecked();
    }
    res.trim_end().to_string()
}
pub fn read_all(&mut self) -> String {
    let mut res = String::new();
    unsafe { self.input.read_to_string(&mut res).unwrap_unchecked() };
    res
}
pub fn read_char(&mut self) -> char {
    self.input
        .by_ref()
        .bytes()
        .map(|b| b.expect("could not read bytes in io read operation"))
        .find(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t' && b != b',')
        .unwrap() as char
}
pub fn chars(&mut self) -> Vec<char> {
    self.r::<String>().chars().collect()
}
pub fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
    (0..n).map(|_| self.r::<T>()).collect()
}
pub fn line_io(
    &mut self,
) -> impl std::iter::Iterator<Item = IoInner<Cursor<String>, Stdout>> {
    let file = self.read_all();
    file.lines()
        .map(move |line| IoInner::from_string(line.to_string()))
        .collect::<Vec<IoInner<Cursor<String>, Stdout>>>()
        .into_iter()
}
pub fn blocks(&mut self) -> Vec<IoInner<Cursor<String>, Stdout>> {
    self.split("\n\n")
}
pub fn split(&mut self, pattern: &str) -> Vec<IoInner<Cursor<String>, Stdout>> {
    let file = self.read_all();
    file.split(pattern)
        .map(move |line| IoInner::from_string(line.to_string()))
        .collect::<Vec<IoInner<Cursor<String>, Stdout>>>()
}
pub fn w<T: Display>(&mut self, t: T) {
    unsafe { write!(&mut self.output, "{t}").unwrap_unchecked() };
}
pub fn wl<T: Display>(&mut self, t: T) {
    self.w(t);
    self.nl();
    self.flush();
}
pub fn nl(&mut self) {
    self.w('\n');
}
pub fn flush(&mut self) {
    unsafe { self.output.flush().unwrap_unchecked() }
}
}
#[macro_export]
macro_rules! wf {
($($arg:tt)*) => {
    {
        let io = unsafe {
            IO.as_mut().unwrap_unchecked()
        };
        io.w(format!($($arg)*));
        io.nl();
    }
};
}
#[macro_export]
macro_rules! w {
($v:expr) => {
    {
        let io = unsafe {
            IO.as_mut().unwrap_unchecked()
        };
        io.w($v);io.nl();
    }
};
($($v:expr);*, $l:expr) => {
    {
        let io = unsafe {
            IO.as_mut().unwrap_unchecked()
        };
        $(
            io.w($v);
            io.w(' ');
        )*
        io.w($l);
        io.nl();
    }
};
($($v:expr),*) => {
    {
        let io = unsafe {
            IO.as_mut().unwrap_unchecked()
        };
        $(
            io.w($v);
            io.w(' ');
        )*
        io.nl();
    }
}
}
#[macro_export]
macro_rules! r {
($T:ty) => {
    {
        let io = unsafe {
            IO.as_mut().unwrap_unchecked()
        };
        io.r::<$T>()
    }
};
($($T:ty),*) => {
    {
        let io = unsafe {
        IO.as_mut().unwrap_unchecked()
        };
        ($(
        io.r::<$T>()
        ),*)
    }
}
}
#[macro_export]
macro_rules! init {
($val:expr, $($dims:expr);+) => {
{
    let mut temp_vec = Vec::new();
    build_vec!(&mut temp_vec, $val, $($dims),+);
    temp_vec
}
};
}

#[macro_export]
macro_rules! build_vec {
($vec:expr, $val:expr, $dim:expr) => {{
    for _ in 0..$dim {
        $vec.push($val);
    }
}};

($vec:expr, $val:expr, $dim:expr, $($rest:expr),+) => {{
    for _ in 0..$dim {
        let mut sub_vec = Vec::new();
        build_vec!(&mut sub_vec, $val, $($rest),+);
        $vec.push(sub_vec);
    }
}};
}
}
