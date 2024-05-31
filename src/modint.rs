type N = Mod<compile_error!()>;

// type N = Mod<156859239850691603>;
// const K: N = N::new(3414519959);

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
    pub const fn pow(self, mut e: usize) -> Self {
        let mut res = Self(1);
        let mut cur = self;
        while e > 0 {
            if e & 1 == 1 {
                res = Self(((res.0 as u128 * cur.0 as u128) % MOD as u128) as usize);
            }
            cur = Self(((cur.0 as u128 * cur.0 as u128) % MOD as u128) as usize);
            e >>= 1;
        }
        res
    }
    pub const fn inv(self) -> Self {
        self.pow(MOD - 2)
    }
}
impl<const MOD: usize> std::iter::Sum for Mod<MOD> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}
impl<const MOD: usize> std::iter::Product for Mod<MOD> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a * b)
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
impl<const MOD: usize> Neg for Mod<MOD> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(MOD - self.0)
    }
}
impl<const MOD: usize> Neg for &Mod<MOD> {
    type Output = Mod<MOD>;

    fn neg(self) -> Self::Output {
        Self::Output::new(MOD - self.0)
    }
}
macro_rules! impl_mod_rest {
    ($($trait:ident::$fn:ident:$trait_assign:ident::$fn_assign:ident;)*) => {
        $(
            impl<const MOD: usize> $trait<&Mod<MOD>> for Mod<MOD> {
                type Output = Mod<MOD>;

                fn $fn(self, rhs: &Mod<MOD>) -> Self::Output {
                    $trait::$fn(self, *rhs)
                }
            }
            impl<const MOD: usize> $trait<&Mod<MOD>> for &Mod<MOD> {
                type Output = Mod<MOD>;

                fn $fn(self, rhs: &Mod<MOD>) -> Self::Output {
                    $trait::$fn(*self, *rhs)
                }
            }
            impl<const MOD: usize> $trait<Mod<MOD>> for &Mod<MOD> {
                type Output = Mod<MOD>;

                fn $fn(self, rhs: Mod<MOD>) -> Self::Output {
                    $trait::$fn(*self, rhs)
                }
            }
            impl<const MOD: usize> $trait<&usize> for Mod<MOD> {
                type Output = Mod<MOD>;

                fn $fn(self, rhs: &usize) -> Self::Output {
                    $trait::$fn(self, *rhs)
                }
            }
            impl<const MOD: usize> $trait<&usize> for &Mod<MOD> {
                type Output = Mod<MOD>;

                fn $fn(self, rhs: &usize) -> Self::Output {
                    $trait::$fn(*self, *rhs)
                }
            }
            impl<const MOD: usize> $trait<usize> for &Mod<MOD> {
                type Output = Mod<MOD>;

                fn $fn(self, rhs: usize) -> Self::Output {
                    $trait::$fn(*self, rhs)
                }
            }
            impl<const MOD: usize> $trait_assign for Mod<MOD> {
                fn $fn_assign(&mut self, rhs: Mod<MOD>) {
                    *self = $trait::$fn(*self, rhs);
                }
            }
            impl<const MOD: usize> $trait_assign<&Mod<MOD>> for Mod<MOD> {
                fn $fn_assign(&mut self, rhs: &Mod<MOD>) {
                    *self = $trait::$fn(*self, rhs);
                }
            }
            impl<const MOD: usize> $trait_assign<usize> for Mod<MOD> {
                fn $fn_assign(&mut self, rhs: usize) {
                    *self = $trait::$fn(*self, rhs);
                }
            }
            impl<const MOD: usize> $trait_assign<&usize> for Mod<MOD> {
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
