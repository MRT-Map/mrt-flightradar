use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Clone, Debug, Serialize, Deserialize, Eq)]
pub struct Airway {
    pub waypoint1: SmolStr,
    pub waypoint2: SmolStr,
}
impl PartialEq for Airway {
    fn eq(&self, other: &Self) -> bool {
        (self.waypoint2 == other.waypoint1 && self.waypoint1 == other.waypoint2)
            || (self.waypoint2 == other.waypoint2 && self.waypoint1 == other.waypoint1)
    }
}
