use anyhow::{anyhow, Result};
use glam::{vec2, Vec2};

pub mod airport;
pub mod iter;
pub mod path;
pub mod timetable;
pub mod vec;
pub mod waypoint;

pub type Angle = f32;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LMR {
    Left,
    Middle,
    Right,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FMB {
    Front,
    Middle,
    Back,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Rotation {
    Clockwise,
    Anticlockwise,
}

impl Rotation {
    pub fn opp(&self) -> Self {
        match self {
            Rotation::Clockwise => Rotation::Anticlockwise,
            Rotation::Anticlockwise => Rotation::Clockwise,
        }
    }
}

/// Note: Assumes no commas in value
#[inline]
pub fn from_csv(str: &str) -> Vec<Vec<&str>> {
    str.split('\n')
        .map(|a| a.split(',').map(|a| a.trim()).collect())
        .collect()
}

#[inline]
pub fn coords_to_vec(str: &str) -> Result<Vec2> {
    let mut ls = str.split(' ');
    let (x, y) = (
        ls.next().ok_or_else(|| anyhow!("No `x` value"))?,
        ls.next().ok_or_else(|| anyhow!("No `y` value"))?,
    );
    Ok(vec2(
        x.parse()
            .map_err(|err| anyhow!("Error parsing `{x}`: `{err}`"))?,
        -y.parse()
            .map_err(|err| anyhow!("Error parsing `{y}`: `{err}`"))?,
    ))
}
