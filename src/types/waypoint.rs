use glam::Vec2;
use smol_str::SmolStr;

use crate::types::vec::Pos;

pub struct Waypoint {
    pub name: SmolStr,
    pub coords: Pos<Vec2>,
}
