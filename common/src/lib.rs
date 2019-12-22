pub mod backtracking;
pub mod expression;
pub mod intcode;
pub mod intcode2;
pub mod intcode_decompile;
pub mod intcode_jit;

use num::{Num, Signed};
use std::ops::BitAnd;

/*pub fn gcd(a: i64, b: i64) -> i64 {
    let a = a.abs();
    let b = b.abs();
    let (mut a, mut b) = if a > b { (a, b) } else { (b, a) };

    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}*/

pub fn gcd<T>(a: T, b: T) -> T
where
    T: Copy + Num + Signed + PartialOrd,
{
    let a = a.abs();
    let b = b.abs();
    let (mut a, mut b) = if a > b { (a, b) } else { (b, a) };

    while !b.is_zero() {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Copy + Num + Signed + PartialOrd,
{
    (a * b).abs() / gcd(a, b)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ModularValue<T> {
    value: T,
    modulo: T,
}

impl<T> ModularValue<T>
where
    T: std::fmt::Debug + Copy + Num + Signed,
{
    pub fn new(value: T, modulo: T) -> Self {
        ModularValue {
            value: Self::modulo(value, modulo),
            modulo,
        }
    }

    fn modulo(x: T, m: T) -> T {
        let y = x % m;
        if y.is_negative() {
            (y + m) % m
        } else {
            y
        }
    }
}

impl<T> ModularValue<T>
where
    T: std::fmt::Debug
        + Copy
        + Num
        + Signed
        + BitAnd<Output = T>
        + std::ops::Shr<Output = T>
        + PartialOrd,
{
    pub fn pow(self, rhs: T) -> Self {
        let mut a = self;
        let mut b = rhs;
        let mut r = ModularValue::new(
            if self.modulo.is_one() {
                T::zero()
            } else {
                T::one()
            },
            self.modulo,
        );
        while b.is_positive() {
            if (b & T::one()) == T::one() {
                r = r * a;
            }
            b = b >> T::one();
            a = a * a;
        }
        r
    }

    pub fn inv(self) -> Option<Self> {
        let g = gcd(self.value, self.modulo);
        if g.is_one() {
            Some(self.pow(self.modulo - T::one() - T::one()))
        } else {
            None
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for ModularValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> std::ops::Add for ModularValue<T>
where
    T: std::fmt::Debug + Copy + Num + Signed,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        debug_assert_eq!(self.modulo, rhs.modulo);
        ModularValue {
            value: Self::modulo(self.value + rhs.value, self.modulo),
            modulo: self.modulo,
        }
    }
}

impl<T> std::ops::Sub for ModularValue<T>
where
    T: std::fmt::Debug + Copy + Num + Signed,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        debug_assert_eq!(self.modulo, rhs.modulo);
        ModularValue {
            value: Self::modulo(self.value - rhs.value, self.modulo),
            modulo: self.modulo,
        }
    }
}

impl<T> std::ops::Mul for ModularValue<T>
where
    T: std::fmt::Debug + Copy + Num + Signed,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        debug_assert_eq!(self.modulo, rhs.modulo);
        ModularValue {
            value: Self::modulo(self.value * rhs.value, self.modulo),
            modulo: self.modulo,
        }
    }
}

impl<T> std::ops::Div for ModularValue<T>
where
    T: std::fmt::Debug
        + Copy
        + Num
        + Signed
        + BitAnd<Output = T>
        + std::ops::Shr<Output = T>
        + PartialOrd,
{
    type Output = Option<Self>;
    fn div(self, rhs: Self) -> Option<Self> {
        Some(self * rhs.inv()?)
    }
}

impl<T> std::ops::Add<T> for ModularValue<T>
where
    T: std::fmt::Debug + Copy + Num + Signed,
{
    type Output = ModularValue<T>;
    fn add(self, rhs: T) -> Self {
        ModularValue {
            value: ModularValue::modulo(self.value + rhs, self.modulo),
            modulo: self.modulo,
        }
    }
}

impl<T> std::ops::Sub<T> for ModularValue<T>
where
    T: std::fmt::Debug + Copy + Num + Signed,
{
    type Output = ModularValue<T>;
    fn sub(self, rhs: T) -> Self {
        ModularValue {
            value: ModularValue::modulo(self.value - rhs, self.modulo),
            modulo: self.modulo,
        }
    }
}

impl<T> std::ops::Mul<T> for ModularValue<T>
where
    T: std::fmt::Debug + Copy + Num + Signed,
{
    type Output = ModularValue<T>;
    fn mul(self, rhs: T) -> Self {
        ModularValue {
            value: ModularValue::modulo(self.value * rhs, self.modulo),
            modulo: self.modulo,
        }
    }
}

impl<T: PartialEq> PartialEq<T> for ModularValue<T> {
    fn eq(&self, other: &T) -> bool {
        self.value == *other
    }
}
