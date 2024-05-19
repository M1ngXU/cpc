type N = Mod<compile_error!()>;

use std::ops::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Mod<const MOD: usize>(pub usize);
#[allow(unused)]
impl<const MOD: usize> Mod<MOD> {
    const ZERO: Self = Self::new(0);
    const NEG_ONE: Self = Self::newi(-1);
    const ONE: Self = Self::new(1);
    const TWO: Self = Self::new(2);

    pub const fn newi(x: isize) -> Self {
        Self::new(((x % MOD as isize) + MOD as isize) as usize)
    }

    pub const fn new(x: usize) -> Self {
        Self(x % MOD)
    }
    pub fn pow(self, mut e: usize) -> Self {
        let mut res = Self(1);
        let mut cur = self;
        while e > 0 {
            if e & 1 == 1 {
                res *= cur;
            }
            cur *= cur;
            e >>= 1;
        }
        res
    }
    pub fn inv(self) -> Self {
        self.pow(MOD - 2)
    }
}
impl<const MOD: usize> std::fmt::Display for Mod<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad_integral(true, "", &self.0.to_string())
    }
}
impl<const MOD: usize> AsRef<usize> for Mod<MOD> {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}
impl<const MOD: usize> AsMut<usize> for Mod<MOD> {
    fn as_mut(&mut self) -> &mut usize {
        &mut self.0
    }
}
impl<const MOD: usize> Add for Mod<MOD> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self((self.0 + rhs.0) % MOD)
    }
}
impl<const MOD: usize> Add<usize> for Mod<MOD> {
    type Output = Self;
    fn add(self, rhs: usize) -> Self {
        Self((self.0 + rhs) % MOD)
    }
}
impl<const MOD: usize> AddAssign for Mod<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl<const MOD: usize> AddAssign<usize> for Mod<MOD> {
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs;
    }
}
impl<const MOD: usize> Sub for Mod<MOD> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self((self.0 + MOD - rhs.0) % MOD)
    }
}
impl<const MOD: usize> Sub<usize> for Mod<MOD> {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self {
        Self((self.0 + MOD - rhs) % MOD)
    }
}
impl<const MOD: usize> SubAssign for Mod<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl<const MOD: usize> SubAssign<usize> for Mod<MOD> {
    fn sub_assign(&mut self, rhs: usize) {
        *self = *self - rhs;
    }
}
impl<const MOD: usize> Mul for Mod<MOD> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self(((self.0 as u128 * rhs.0 as u128) % MOD as u128) as usize)
    }
}
impl<const MOD: usize> Mul<usize> for Mod<MOD> {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self {
        Self(((self.0 as u128 * rhs as u128) % MOD as u128) as usize)
    }
}
impl<const MOD: usize> MulAssign for Mod<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl<const MOD: usize> MulAssign<usize> for Mod<MOD> {
    fn mul_assign(&mut self, rhs: usize) {
        *self = *self * rhs;
    }
}
impl<const MOD: usize> Div for Mod<MOD> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.inv()
    }
}
impl<const MOD: usize> Div<usize> for Mod<MOD> {
    type Output = Self;
    fn div(self, rhs: usize) -> Self {
        self * Mod::new(rhs).inv()
    }
}
impl<const MOD: usize> DivAssign for Mod<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
impl<const MOD: usize> DivAssign<usize> for Mod<MOD> {
    fn div_assign(&mut self, rhs: usize) {
        *self = *self / rhs;
    }
}
impl<const MOD: usize> Neg for Mod<MOD> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(MOD - self.0)
    }
}
