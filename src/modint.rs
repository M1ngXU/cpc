// const MOD: usize = 998_244_353;
// const MOD: usize = 1_000_000_007;
type N = Mod;

use std::ops::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Mod(pub usize);
#[allow(unused)]
impl Mod {
    const ZERO: Self = Self::new(0);
    const NEG_ONE: Self = Self::newi(-1);
    const ONE: Self = Self::new(1);
    const TWO: Self = Self::new(2);
    const NEG_TWO: Self = Self::newi(-2);
    const INV_TWO: Self = Self::new(2).inv();

    pub const fn newi(x: isize) -> Self {
        Self::new(((x % MOD as isize) + MOD as isize) as usize)
    }

    fn new2(x: usize) -> Self {
        debug_assert!(x < MOD << 1);
        if x >= MOD {
            Self(x - MOD)
        } else {
            Self(x)
        }
    }

    pub const fn new(x: usize) -> Self {
        Self(x % MOD)
    }
    pub const fn pow(self, mut e: usize) -> Self {
        let mut res = Self(1);
        let mut cur = self;
        while e > 0 {
            if e & 1 == 1 {
                res = res.fast_mul(cur);
            }
            cur = cur.fast_mul(cur);
            e >>= 1;
        }
        res
    }
    pub const fn inv(self) -> Self {
        #[cfg(debug_assertions)]
        if self.0 == 0 {
            panic!("Division by `0`.");
        }
        self.pow(MOD - 2)
    }

    pub const fn fast_mul(self, rhs: Self) -> Self {
        if MOD < u32::MAX as usize {
            Self((self.0 * rhs.0) % MOD)
        } else {
            Self(((self.0 as u128 * rhs.0 as u128) % MOD as u128) as usize)
        }
    }
}
impl std::iter::Sum for Mod {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}
impl std::iter::Product for Mod {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}
impl std::fmt::Display for Mod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad_integral(true, "", &self.0.to_string())
    }
}
impl AsRef<usize> for Mod {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}
impl AsMut<usize> for Mod {
    fn as_mut(&mut self) -> &mut usize {
        &mut self.0
    }
}
impl Add for Mod {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new2(self.0 + rhs.0)
    }
}
impl Add<usize> for Mod {
    type Output = Self;
    fn add(self, rhs: usize) -> Self {
        self + Self::new(rhs)
    }
}
impl Sub for Mod {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new2(self.0 + MOD - rhs.0)
    }
}
impl Sub<usize> for Mod {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self {
        self - Self::new(rhs)
    }
}
impl Mul for Mod {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.fast_mul(rhs)
    }
}
impl Mul<usize> for Mod {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self {
        if MOD < u32::MAX as usize {
            Self((self.0 * rhs) % MOD)
        } else {
            Self(((self.0 as u128 * rhs as u128) % MOD as u128) as usize)
        }
    }
}
impl Div for Mod {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.inv()
    }
}
impl Div<usize> for Mod {
    type Output = Self;
    fn div(self, rhs: usize) -> Self {
        self * Mod::new(rhs).inv()
    }
}
impl Neg for Mod {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self.0 == 0 {
            self
        } else {
            Self(MOD - self.0)
        }
    }
}
impl Neg for &Mod {
    type Output = Mod;

    fn neg(self) -> Self::Output {
        -*self
    }
}
macro_rules! impl_mod_rest {
    ($($trait:ident::$fn:ident:$trait_assign:ident::$fn_assign:ident;)*) => {
        $(
            impl $trait<&Mod> for Mod {
                type Output = Mod;

                fn $fn(self, rhs: &Mod) -> Self::Output {
                    $trait::$fn(self, *rhs)
                }
            }
            impl $trait<&Mod> for &Mod {
                type Output = Mod;

                fn $fn(self, rhs: &Mod) -> Self::Output {
                    $trait::$fn(*self, *rhs)
                }
            }
            impl $trait<Mod> for &Mod {
                type Output = Mod;

                fn $fn(self, rhs: Mod) -> Self::Output {
                    $trait::$fn(*self, rhs)
                }
            }
            impl $trait<&usize> for Mod {
                type Output = Mod;

                fn $fn(self, rhs: &usize) -> Self::Output {
                    $trait::$fn(self, *rhs)
                }
            }
            impl $trait<&usize> for &Mod {
                type Output = Mod;

                fn $fn(self, rhs: &usize) -> Self::Output {
                    $trait::$fn(*self, *rhs)
                }
            }
            impl $trait<usize> for &Mod {
                type Output = Mod;

                fn $fn(self, rhs: usize) -> Self::Output {
                    $trait::$fn(*self, rhs)
                }
            }
            impl $trait_assign for Mod {
                fn $fn_assign(&mut self, rhs: Mod) {
                    *self = $trait::$fn(*self, rhs);
                }
            }
            impl $trait_assign<&Mod> for Mod {
                fn $fn_assign(&mut self, rhs: &Mod) {
                    *self = $trait::$fn(*self, rhs);
                }
            }
            impl $trait_assign<usize> for Mod {
                fn $fn_assign(&mut self, rhs: usize) {
                    *self = $trait::$fn(*self, rhs);
                }
            }
            impl $trait_assign<&usize> for Mod {
                fn $fn_assign(&mut self, rhs: &usize) {
                    *self = $trait::$fn(*self, rhs);
                }
            }
        )*
    };
}
impl_mod_rest! {
    Add::add:AddAssign::add_assign;
    Sub::sub:SubAssign::sub_assign;
    Mul::mul:MulAssign::mul_assign;
    Div::div:DivAssign::div_assign;
}
