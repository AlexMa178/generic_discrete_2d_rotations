mod ray;
pub use ray::*;

use std::ops::{ Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign };
use std::f32::consts::{ TAU };

/// A Rot represents an angle.
/// `N` may not be zero.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Rot<const N: u16> {
    n: u16,
}
impl<const N: u16> Rot<N> {

    pub fn all() -> Vec<Rot<N>> {
        (0..N).map(|i| Rot { n: i }).collect()
    }
    
    /// Gives None if `angle`, once normalized to `0..=359`, is not a multiple of `N`.
    pub const fn from_deg_exact(angle: u16) -> Option<Self> {
        let norm_angle = angle % 360;
        if norm_angle.is_multiple_of(N) { Some(Self { n: norm_angle / N }) } else { None }
    }

    /// Snaps to the closest valid `Rot<N>`.
    pub const fn from_deg(angle: f32) -> Self {
        let quantize = (angle / (360. / N as f32)).round() as i32;
        let normalize = quantize.rem_euclid(N as i32) as u16;
        Self { n: normalize }
    }

    /// Snaps to the closest valid `Rot<N>`.
    pub const fn from_rad(angle: f32) -> Self {
        let quantize = (angle / (TAU / N as f32)).round() as i32;
        let normalize = quantize.rem_euclid(N as i32) as u16;
        Self { n: normalize }
    }

    /// # Panics
    /// Panics if `360` is not a multiple of `N`.
    pub const fn to_deg_exact(self) -> u16 {
        if !360u16.is_multiple_of(N) { panic!("failed to_deg_exact"); }
        self.n * (360 / N)
    }
    
    pub const fn to_deg(self) -> f32 {
        self.n as f32 * (360. / N as f32)
    }

    pub const fn to_rad(self) -> f32 {
        self.n as f32 * (TAU / N as f32)
    }

    /// Gives `None` if `self` cannot be divided.
    /// For example: `Rot::R8_90.split_as::<4>()` gives `Some(Rot::R4_90)`, but `Rot::R8_45.split_as::<4>()` gives `None`.
    /// # Panics
    /// Panics if `N` is not a multiple of `M`.
    pub const fn split_as<const M: u16>(self) -> Option<Rot<M>> {
        if !N.is_multiple_of(M) { panic!("failed split_as"); }
        if self.n.is_multiple_of(N / M) { Some(Rot { n: self.n / (N / M) }) } else { None }
    }

    /// # Panics
    /// Panics if `M` is not a multiple of `N`.
    pub const fn embed_as<const M: u16>(self) -> Rot<M> {
        if !M.is_multiple_of(N) { panic!("failed embed_as"); }
        Rot { n: self.n * (M / N) }
    }

}
impl Rot<4> {

    pub const R4_0  : Self = Self { n: 0 };
    pub const R4_90 : Self = Self { n: 1 };
    pub const R4_180: Self = Self { n: 2 };
    pub const R4_270: Self = Self { n: 3 };

    /// Useful for doing an exhaustive match on a `Rot<4>`.
    pub const fn matchable_4(self) -> R4 {
        match self.n {
            0 => R4::D0,
            1 => R4::D90,
            2 => R4::D180,
            3 => R4::D270,
            _ => unreachable!(),
        }
    }

}
impl Rot<8> {

    pub const R8_0  : Self = Self { n: 0 };
    pub const R8_45 : Self = Self { n: 1 };
    pub const R8_90 : Self = Self { n: 2 };
    pub const R8_135: Self = Self { n: 3 };
    pub const R8_180: Self = Self { n: 4 };
    pub const R8_225: Self = Self { n: 5 };
    pub const R8_270: Self = Self { n: 6 };
    pub const R8_315: Self = Self { n: 7 };

    /// Useful for doing an exhaustive match on a `Rot<8>`.
    pub const fn matchable_8(self) -> R8 {
        match self.n {
            0 => R8::D0,
            1 => R8::D45,
            2 => R8::D90,
            3 => R8::D135,
            4 => R8::D180,
            5 => R8::D225,
            6 => R8::D270,
            7 => R8::D315,
            _ => unreachable!(),
        }
    }

}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum R4 {
    D0, D90, D180, D270
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum R8 {
    D0, D45, D90, D135, D180, D225, D270, D315
}

const fn neg_const<const N: u16>(r: Rot<N>) -> Rot<N> {
    Rot { n: (N - r.n) % N }
}
const fn add_const<const N: u16>(a: Rot<N>, b: Rot<N>) -> Rot<N> {
    Rot { n: (a.n + b.n) % N }
}
const fn sub_const<const N: u16>(a: Rot<N>, b: Rot<N>) -> Rot<N> {
    Rot { n: (a.n + N - b.n) % N }
}
const fn mul_const<const N: u16>(r: Rot<N>, factor: u16) -> Rot<N> {
    Rot { n: (r.n * factor) % N }
}

impl<const N: u16> Neg for Rot<N> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        neg_const::<N>(self)
    }
}
impl<const N: u16> Add for Rot<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        add_const::<N>(self, rhs)
    }
}
impl<const N: u16> AddAssign for Rot<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = add_const::<N>(*self, rhs);
    }
}
impl<const N: u16> Sub for Rot<N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        sub_const::<N>(self, rhs)
    }
}
impl<const N: u16> SubAssign for Rot<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = sub_const::<N>(*self, rhs);
    }
}
impl<const N: u16> Mul<u16> for Rot<N> {
    type Output = Rot<N>;
    fn mul(self, rhs: u16) -> Self::Output {
        mul_const::<N>(self, rhs)
    }
}
impl<const N: u16> Mul<Rot<N>> for u16 {
    type Output = Rot<N>;
    fn mul(self, rhs: Rot<N>) -> Self::Output {
        mul_const::<N>(rhs, self)
    }
}
impl<const N: u16> MulAssign<u16> for Rot<N> {
    fn mul_assign(&mut self, rhs: u16) {
        *self = mul_const::<N>(*self, rhs);
    }
}