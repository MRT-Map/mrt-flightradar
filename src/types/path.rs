use glam::Vec2;

use crate::types::{
    Angle,
    vec::{FromLoc, Pos},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Path {
    Straight(FromLoc<Vec2>),
    Curve {
        centre: Pos<Vec2>,
        from: Pos<Vec2>,
        angle: Angle,
    },
}

pub trait PathExt {
    fn length(&self) -> f32;
}

impl PathExt for Path {
    fn length(&self) -> f32 {
        match &self {
            Path::Straight(fl) => fl.vec.length(),
            Path::Curve {
                from,
                centre,
                angle,
            } => (*centre - *from).length() * angle.abs(),
        }
    }
}

impl PathExt for Vec<Path> {
    fn length(&self) -> f32 {
        self.iter().map(Path::length).sum()
    }
}
