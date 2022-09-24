pub mod vec;
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
    pub fn opp(&self) -> Self {
        match self {
            Rotation::Clockwise => Rotation::Anticlockwise,
            Rotation::Anticlockwise => Rotation::Clockwise,
        }
    }
}
