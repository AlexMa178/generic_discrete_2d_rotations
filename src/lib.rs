use std::cmp::Ordering;
use std::ops::{ Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign };
use std::f32::consts::{ TAU };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotFrom {
    PosY, NegY, PosX, NegX
}

impl RotFrom {

    pub const fn angle_to(self, other: RotFrom, dir: RotDir) -> Rot<4> {
        const fn from_pos_x_ccw(f: RotFrom) -> Rot<4> {
            match f {
                RotFrom::PosX => Rot::R4_0,
                RotFrom::PosY => Rot::R4_90,
                RotFrom::NegX => Rot::R4_180,
                RotFrom::NegY => Rot::R4_270,
            }
        }
        let diff_ccw = sub_const(from_pos_x_ccw(self), from_pos_x_ccw(other));
        match dir {
            RotDir::Clockwise => diff_ccw,
            RotDir::CounterClockwise => neg_const(diff_ccw),
        }
    }
    
}
 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotDir {
    Clockwise, CounterClockwise
}
impl RotDir {

    pub const fn opposite(self) -> Self {
        match self {
            Self::Clockwise => Self::CounterClockwise,
            Self::CounterClockwise => Self::Clockwise,
        }
    }

}

/// `N` may not be zero.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rot<const N: u16> {
    n: u16,
}
impl<const N: u16> Rot<N> {

    
    /// Gives None if `angle`, once normalized to `0..=359`, is not a multiple of `N`.
    pub const fn from_deg_exact(angle: u16) -> Option<Rot<N>> {
        let norm_angle = angle % 360;
        if norm_angle.is_multiple_of(N) { Some(Rot { n: norm_angle / N }) } else { None }
    }

    /// 'Snaps' to the closest valid `Rot<N>`.
    pub const fn from_deg(angle: f32) -> Rot<N> {
        let quantize = (angle / (360. / N as f32)).round() as i32;
        let normalize = quantize.rem_euclid(N as i32) as u16;
        Rot { n: normalize }
    }

    /// 'Snaps' to the closest valid `Rot<N>`.
    pub const fn from_rad(angle: f32) -> Rot<N> {
        let quantize = (angle / (TAU / N as f32)).round() as i32;
        let normalize = quantize.rem_euclid(N as i32) as u16;
        Rot { n: normalize }
    }

    /// Panics if `N` is not `1`, `2`, `4`, or `8`.
    /// Gives `None` if `dx` and `dy` don't point in a valid direction.
    /// For example: it gives `None` if `N` is `4` and both `dx` and `dy` are `Ordering::Greater`, because that would be diagonal.
    pub fn from_signs(from: RotFrom, dir: RotDir, dx: Ordering, dy: Ordering) -> Option<Rot<N>> {
        use { Ordering::Less as Neg, Ordering::Equal as Zer, Ordering::Greater as Pos };
        let rot_8_pos_y_cw = match (dx, dy) {
            (Zer, Pos) => Rot::R8_0,
            (Pos, Pos) => Rot::R8_45, 
            (Pos, Zer) => Rot::R8_90,
            (Pos, Neg) => Rot::R8_135,
            (Zer, Neg) => Rot::R8_180,
            (Neg, Neg) => Rot::R8_225,
            (Neg, Zer) => Rot::R8_270,
            (Neg, Pos) => Rot::R8_315,
            (Zer, Zer) => return None,
        };
        let rot_8 = rot_8_pos_y_cw.change_relative_to(RotFrom::PosY, RotDir::Clockwise, from, dir);
        rot_8.split_as::<N>()
    }

    /// Panics if `N` is not a multiple of `4`.
    /// Gives `None` is `dx` and `dy` are `0`.
    /// 'Snaps' to the closest valid `Rot<N>`.
    pub fn from_vector(from: RotFrom, dir: RotDir, dx: f32, dy: f32) -> Option<Rot<N>> {
        if f32::hypot(dx, dy) <= f32::EPSILON { return None; }
        let rot_pos_x_ccw = Rot::<N>::from_rad(f32::atan2(dy, dx));
        Some(rot_pos_x_ccw.change_relative_to(RotFrom::PosX, RotDir::CounterClockwise, from, dir))
    }

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

    /// Panics if `N` is not a multiple of `M`.
    /// Gives `None` if `self` cannot be divided.
    /// For example: `Rot::R8_90.split_as::<4>()` gives `Some(Rot::R4_90)`, but `Rot::R8_45.split_as::<4>()` gives `None`.
    pub const fn split_as<const M: u16>(self) -> Option<Rot<M>> {
        if !N.is_multiple_of(M) { panic!("failed split_as"); }
        if self.n.is_multiple_of(N / M) { Some(Rot { n: self.n / (N / M) }) } else { None }
    }

    /// Panics if `M` is not a multiple of `N`.
    pub const fn embed_as<const M: u16>(self) -> Rot<M> {
        if !M.is_multiple_of(N) { panic!("failed embed_as"); }
        Rot { n: self.n * (M / N) }
    }

    /// Panics if `N` is not a multiple of `4`.
    pub fn change_relative_to(self, old_from: RotFrom, old_dir: RotDir, new_from: RotFrom, new_dir: RotDir) -> Self {
        pub const fn old_to_pos_x_ccw<const N: u16>(r: Rot<N>, old_from: RotFrom, old_dir: RotDir) -> Rot<N> {
            let d = old_from.angle_to(RotFrom::PosX, old_dir).embed_as::<N>();
            match old_dir {
                RotDir::Clockwise => sub_const(d, r),
                RotDir::CounterClockwise => sub_const(r, d),
            }
        }
        pub const fn pos_x_ccw_to_new<const N: u16>(r: Rot<N>, new_from: RotFrom, new_dir: RotDir) -> Rot<N> {
            let d = RotFrom::PosX.angle_to(new_from, RotDir::CounterClockwise).embed_as::<N>();
            match new_dir {
                RotDir::Clockwise => sub_const(d, r),
                RotDir::CounterClockwise => sub_const(r, d),
            }
        }
        pos_x_ccw_to_new(old_to_pos_x_ccw(self, old_from, old_dir), new_from, new_dir)
    }

    /// Panics if `N` is not a multiple of `4`.
    pub fn unit_vector(self, from: RotFrom, dir: RotDir) -> [ f32; 2 ] {
        let (x, y) = self.change_relative_to(from, dir, RotFrom::PosY, RotDir::Clockwise).to_rad().sin_cos();
        [ x, y ]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum R4 {
    D0, D90, D180, D270
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
impl<const N: u16> Sub for Rot<N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        sub_const::<N>(self, rhs)
    }
}
impl<const N: u16> Mul<u16> for Rot<N> {
    type Output = Self;
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
impl<const N: u16> AddAssign for Rot<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = add_const::<N>(*self, rhs)
    }
}
impl<const N: u16> SubAssign for Rot<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = sub_const::<N>(*self, rhs);
    }
}
impl<const N: u16> MulAssign<u16> for Rot<N> {
    fn mul_assign(&mut self, rhs: u16) {
        *self = mul_const::<N>(*self, rhs);
    }
}