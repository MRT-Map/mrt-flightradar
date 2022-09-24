use glam::Vec2;
use smallvec::SmallVec;
use smol_str::SmolStr;

use crate::types::vec::Pos;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum RunwayLength {
    Large,
    Small,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Runway {
    pub start: Pos<Vec2>,
    pub end: Pos<Vec2>,
    pub direction: (SmolStr, SmolStr),
    pub length: RunwayLength,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PlaneFacilityType {
    Airport,
    Airfield,
}

#[derive(Clone, PartialEq, Debug)]
pub enum AirFacility {
    Heliport {
        code: SmolStr,
        pad_coord: Pos<Vec2>,
    },
    Airport {
        code: SmolStr,
        ty: PlaneFacilityType,
        runway: SmallVec<[Runway; 1]>,
    },
}
