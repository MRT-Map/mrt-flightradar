use color_eyre::eyre::{eyre, Result};
use glam::{vec2, Vec2};

pub mod iter;
pub mod path;

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
    #[must_use]
    pub const fn opp(&self) -> Self {
        match self {
            Self::Clockwise => Self::Anticlockwise,
            Self::Anticlockwise => Self::Clockwise,
        }
    }
}

/// Note: Assumes no commas in value
#[inline]
#[must_use]
pub fn from_csv(str: &str) -> Vec<Vec<&str>> {
    str.split('\n')
        .map(|a| a.split(',').map(str::trim).collect())
        .collect()
}

#[inline]
pub fn coords_to_vec(str: &str) -> Result<Vec2> {
    let mut ls = str.split(' ');
    let (x, y) = (
        ls.next().ok_or_else(|| eyre!("No `x` value"))?,
        ls.next().ok_or_else(|| eyre!("No `y` value"))?,
    );
    let (x, y) = (
        x.parse::<f32>()
            .map_err(|err| eyre!("Error parsing `x` ({x}): `{err}`"))?,
        -y.parse::<f32>()
            .map_err(|err| eyre!("Error parsing `y` ({y}): `{err}`"))?,
    );
    if x.is_nan() {
        return Err(eyre!("`x` is NaN"));
    }
    if y.is_nan() {
        return Err(eyre!("`y` is NaN"));
    }
    Ok(vec2(x, y))
}
