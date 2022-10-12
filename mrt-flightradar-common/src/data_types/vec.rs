use std::ops::{Add, Neg, Sub};

use glam::Vec2;
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::flight_route::types::{Rotation, FMB, LMR};

pub trait Vector:
    Copy + Add<Self, Output = Self> + Sub<Self, Output = Self> + Neg<Output = Self>
{
}

impl Vector for Vec2 {}

/// Position vector
pub type Pos<T> = T;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FromLoc<T: Vector = Vec2> {
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
    #[inline]
    pub fn rev(self) -> Self {
        Self {
            tail: self.head(),
            vec: -self.vec,
        }
    }
}

impl FromLoc {
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

impl Direction<Vec2> for FromLoc {
    #[inline]
    fn lmr(&self, other: Pos<Vec2>) -> LMR {
        match self.vec.perp_dot(other - self.head()) {
            a if a > 0.0 => LMR::Left,
            a if a == 0.0 || a == -0.0 => LMR::Middle,
            a if a < 0.0 => LMR::Right,
            _ => {
                warn!(?self.vec, ?other, "NaN detected");
                LMR::Middle
            }
        }
    }
    #[inline]
    fn fmb(&self, other: Pos<Vec2>) -> FMB {
        match self.vec.dot(other - self.head()) {
            a if a > 0.0 => FMB::Front,
            a if a == 0.0 || a == -0.0 => FMB::Middle,
            a if a < 0.0 => FMB::Back,
            _ => {
                warn!(?self.vec, ?other, "NaN detected");
                FMB::Middle
            }
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
