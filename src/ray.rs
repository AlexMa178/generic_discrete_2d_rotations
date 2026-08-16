use std::cmp::Ordering;
use std::ops::{ Add, AddAssign, Neg, Sub, SubAssign };

use crate::{ Rot, add_const, neg_const, sub_const };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
 
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// A Ray represents a direction in 2d cartesian space.
pub struct Ray<const N: u16> {
    pos_x_ccw: Rot<N>
}
impl<const N: u16> Ray<N> {

    /// # Panics
    /// Panics if `N` is not a multiple of `4`.
    pub const fn new(rot: Rot<N>, from: RotFrom, dir: RotDir) -> Self {
        let d = from.angle_to(RotFrom::PosX, dir).embed_as::<N>();
        Self { pos_x_ccw: match dir {
            RotDir::Clockwise => sub_const(d, rot),
            RotDir::CounterClockwise => sub_const(rot, d),
        }}
    }

    /// # Panics
    /// Panics if `N` is not a multiple of `4`.
    pub const fn rot(self, from: RotFrom, dir: RotDir) -> Rot<N> {
        let d = RotFrom::PosX.angle_to(from, RotDir::CounterClockwise).embed_as::<N>();
        match dir {
            RotDir::Clockwise => sub_const(d, self.pos_x_ccw),
            RotDir::CounterClockwise => sub_const(self.pos_x_ccw, d),
        }
    }

    /// Gives `None` if `dx` and `dy` don't point in a valid direction.
    /// For example: it gives `None` if `N` is `4` and both `dx` and `dy` are `Ordering::Greater`, because that would be diagonal.
    /// # Panics
    /// Panics if `8` is not a multiple of `N`.
    pub const fn from_signs(dx: Ordering, dy: Ordering) -> Option<Self> {
        use { Ordering::Less as Neg, Ordering::Equal as Zer, Ordering::Greater as Pos };
        let angle_8 = match (dx, dy) {
            (Pos, Zer) => Rot::R8_0,
            (Pos, Pos) => Rot::R8_45, 
            (Zer, Pos) => Rot::R8_90,
            (Neg, Pos) => Rot::R8_135,
            (Neg, Zer) => Rot::R8_180,
            (Neg, Neg) => Rot::R8_225,
            (Zer, Neg) => Rot::R8_270,
            (Pos, Neg) => Rot::R8_315,
            (Zer, Zer) => return None,
        };
        match angle_8.split_as::<N>() {
            None => None,
            Some(angle_n) => Some(Self { pos_x_ccw: angle_n }),
        }
    }

    /// Gives `None` is `dx` and `dy` are `0`.
    /// Snaps to the closest valid `Rot<N>`.
    /// # Panics
    /// Panics if `N` is not a multiple of `4`.
    pub fn from_vector(dx: f32, dy: f32) -> Option<Self> {
        assert!(N.is_multiple_of(4));
        if f32::hypot(dx, dy) <= f32::EPSILON { return None; }
        Some(Self { pos_x_ccw: Rot::<N>::from_rad(f32::atan2(dy, dx)) })
    }

    pub fn unit_vector(self) -> [ f32; 2 ] {
        let (y, x) = self.pos_x_ccw.to_rad().sin_cos();
        [ x, y ]
    }

}

impl<const N: u16> Neg for Ray<N> {
    type Output = Ray<N>;
    fn neg(self) -> Self::Output {
        Ray { pos_x_ccw: neg_const(self.pos_x_ccw) }
    }
}
impl<const N: u16> Add<Rot<N>> for Ray<N> {
    type Output = Ray<N>;
    fn add(self, rhs: Rot<N>) -> Self::Output {
        Ray { pos_x_ccw: add_const(self.pos_x_ccw, rhs) }
    }
}
impl<const N: u16> Add<Ray<N>> for Rot<N> {
    type Output = Ray<N>;
    fn add(self, rhs: Ray<N>) -> Self::Output {
        Ray { pos_x_ccw: add_const(self, rhs.pos_x_ccw) }
    }
}
impl<const N: u16> AddAssign<Rot<N>> for Ray<N> {
    fn add_assign(&mut self, rhs: Rot<N>) {
        self.pos_x_ccw = add_const(self.pos_x_ccw, rhs);
    }
}
impl<const N: u16> Sub<Rot<N>> for Ray<N> {
    type Output = Ray<N>;
    fn sub(self, rhs: Rot<N>) -> Self::Output {
        Ray { pos_x_ccw: sub_const(self.pos_x_ccw, rhs) }
    }
}
impl<const N: u16> SubAssign<Rot<N>> for Ray<N> {
    fn sub_assign(&mut self, rhs: Rot<N>) {
        self.pos_x_ccw = sub_const(self.pos_x_ccw, rhs);
    }
}