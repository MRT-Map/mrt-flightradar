use std::ops::{Add, Neg, Sub};

use glam::Vec2;

use crate::types::{FMB, LMR, Rotation};

pub trait Vector:
    Copy + Add<Self, Output = Self> + Sub<Self, Output = Self> + Neg<Output = Self>
{
}

impl Vector for Vec2 {}

/// Position vector
pub type Pos<T> = T;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FromLoc<T: Vector> {
    pub tail: Pos<T>,
    pub vec: T,
}

impl<T: Vector> FromLoc<T> {
    #[inline]
    pub fn new(tail: Pos<T>, head: Pos<T>) -> Self {
        Self {
            tail,
            vec: head - tail,
        }
    }
    #[inline]
    pub fn head(&self) -> Pos<T> {
        self.tail + self.vec
    }
}

impl FromLoc<Vec2> {
    pub fn intersects(&self, other: Self) -> bool {
        // T1x + p*V1x = T2x + q*V2x
        // T1y + p*V1y = T2y + q*V2y
        let [a, w] = self.tail.to_array();
        let [b, x] = self.vec.to_array();
        let [c, y] = other.tail.to_array();
        let [d, z] = other.vec.to_array();
        let p = (w * d - y * d - z * a + z * c) / (z * b - x * d);
        let q = (y * b - w * b - x * c + x * a) / (x * d - z * b);
        (0.0..=1.0).contains(&p) && (0.0..=1.0).contains(&q)
    }
}

pub trait Direction<T> {
    fn lmr(&self, other: Pos<T>) -> LMR;
    fn fmb(&self, other: Pos<T>) -> FMB;
    fn turning_rot(&self, other: Pos<T>) -> Option<Rotation>;
}

impl Direction<Vec2> for FromLoc<Vec2> {
    #[inline]
    fn lmr(&self, other: Pos<Vec2>) -> LMR {
        match self.vec.perp_dot(other - self.tail) {
            a if a > 0.0 => LMR::Left,
            a if a == 0.0 || a == -0.0 => LMR::Middle,
            a if a < 0.0 => LMR::Right,
            a => panic!("{}", a),
        }
    }
    #[inline]
    fn fmb(&self, other: Pos<Vec2>) -> FMB {
        match self.vec.dot(other - self.tail) {
            a if a > 0.0 => FMB::Front,
            a if a == 0.0 || a == -0.0 => FMB::Middle,
            a if a < 0.0 => FMB::Back,
            a => panic!("{}", a),
        }
    }
    #[inline]
    fn turning_rot(&self, other: Pos<Vec2>) -> Option<Rotation> {
        match self.lmr(other) {
            LMR::Left => Some(Rotation::Anticlockwise),
            LMR::Right => Some(Rotation::Clockwise),
            LMR::Middle => None,
        }
    }
}
