use glam::Vec2;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::data_types::vec::Pos;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Waypoint {
    pub name: SmolStr,
    pub coords: Pos<Vec2>,
}
