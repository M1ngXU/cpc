use std::ops::*;
pub type CT = f64;
fn sqrt(x: CT) -> CT {
    x.sqrt()
}
fn sin_cos(x: CT) -> (CT, CT) {
    x.sin_cos()
}
const PI: CT = std::f64::consts::PI;

#[derive(Clone, Copy, PartialEq)]
pub struct Complex(pub CT, pub CT);
impl std::fmt::Debug for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.0, self.1)
    }
}
impl Complex {
    pub const ZERO: Complex = Complex(0., 0.);
    pub const ONE: Complex = Complex(1., 0.);
    pub const I: Complex = Complex(0., 1.);

    pub fn len_sq(&self) -> CT {
        self.0 * self.0 + self.1 * self.1
    }

    pub fn len(&self) -> CT {
        sqrt(self.len_sq())
    }
}
impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Add<CT> for Complex {
    type Output = Self;

    fn add(self, rhs: CT) -> Self::Output {
        Self(self.0 + rhs, self.1)
    }
}
impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl Sub<CT> for Complex {
    type Output = Self;

    fn sub(self, rhs: CT) -> Self::Output {
        Self(self.0 - rhs, self.1)
    }
}
impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}
impl Mul<CT> for Complex {
    type Output = Self;

    fn mul(self, rhs: CT) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}
impl Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * Self(rhs.0, -rhs.1) / rhs.len_sq()
    }
}
impl Div<CT> for Complex {
    type Output = Self;

    fn div(self, rhs: CT) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}
impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Self::Output {
        Complex(-self.0, -self.1)
    }
}
impl Neg for &Complex {
    type Output = Complex;

    fn neg(self) -> Self::Output {
        Complex(-self.0, -self.1)
    }
}
macro_rules! impl_complex_rest {
    ($($trait:ident::$fn:ident:$trait_assign:ident::$fn_assign:ident;)*) => {
        $(
            impl $trait<&Complex> for Complex {
                type Output = Complex;

                fn $fn(self, rhs: &Complex) -> Self::Output {
                    $trait::$fn(self, *rhs)
                }
            }
            impl $trait<&Complex> for &Complex {
                type Output = Complex;

                fn $fn(self, rhs: &Complex) -> Self::Output {
                    $trait::$fn(*self, *rhs)
                }
            }
            impl $trait<Complex> for &Complex {
                type Output = Complex;

                fn $fn(self, rhs: Complex) -> Self::Output {
                    $trait::$fn(*self, rhs)
                }
            }
            impl $trait<&CT> for Complex {
                type Output = Complex;

                fn $fn(self, rhs: &CT) -> Self::Output {
                    $trait::$fn(self, *rhs)
                }
            }
            impl $trait<&CT> for &Complex {
                type Output = Complex;

                fn $fn(self, rhs: &CT) -> Self::Output {
                    $trait::$fn(*self, *rhs)
                }
            }
            impl $trait<CT> for &Complex {
                type Output = Complex;

                fn $fn(self, rhs: CT) -> Self::Output {
                    $trait::$fn(*self, rhs)
                }
            }
            impl $trait_assign for Complex {
                fn $fn_assign(&mut self, rhs: Complex) {
                    *self = $trait::$fn(*self, rhs);
                }
            }
            impl $trait_assign<&Complex> for Complex {
                fn $fn_assign(&mut self, rhs: &Complex) {
                    *self = $trait::$fn(*self, rhs);
                }
            }
            impl $trait_assign<CT> for Complex {
                fn $fn_assign(&mut self, rhs: CT) {
                    *self = $trait::$fn(*self, rhs);
                }
            }
            impl $trait_assign<&CT> for Complex {
                fn $fn_assign(&mut self, rhs: &CT) {
                    *self = $trait::$fn(*self, rhs);
                }
            }
        )*
    };
}
impl_complex_rest! {
    Add::add:AddAssign::add_assign;
    Sub::sub:SubAssign::sub_assign;
    Mul::mul:MulAssign::mul_assign;
    Div::div:DivAssign::div_assign;
}
