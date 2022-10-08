use glam::Vec2;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use smol_str::SmolStr;

use crate::data_types::vec::Pos;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum RunwayLength {
    Large,
    Small,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Runway {
    pub start: Pos<Vec2>,
    pub end: Pos<Vec2>,
    pub direction: (SmolStr, SmolStr),
    pub length: RunwayLength,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum PlaneFacilityType {
    Airport,
    Airfield,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum AirFacility {
    Heliport {
        code: SmolStr,
        pad_coord: Pos<Vec2>,
    },
    AirshipTerminal {
        code: SmolStr,
        pad_coord: Pos<Vec2>,
    },
    Airport {
        code: SmolStr,
        ty: PlaneFacilityType,
        runways: SmallVec<[Runway; 1]>,
    },
}
impl AirFacility {
    pub fn code(&self) -> &SmolStr {
        match &self {
            AirFacility::Heliport { code, .. } => code,
            AirFacility::AirshipTerminal { code, .. } => code,
            AirFacility::Airport { code, .. } => code,
        }
    }
    pub fn main_coord(&self) -> Option<&Pos<Vec2>> {
        match &self {
            AirFacility::Heliport { pad_coord, .. } => Some(pad_coord),
            AirFacility::AirshipTerminal { pad_coord, .. } => Some(pad_coord),
            AirFacility::Airport { runways, .. } => runways.get(1).map(|r| &r.start),
        }
    }
}
