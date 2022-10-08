use cached::once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::data_types::{airport::AirFacility, airway::Airway, waypoint::Waypoint};

pub mod airport;
pub mod airway;
pub mod time;
pub mod timetable;
pub mod vec;
pub mod waypoint;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawData {
    pub air_facilities: Vec<AirFacility>,
    pub waypoints: Vec<Waypoint>,
    pub airways: Vec<Airway>,
}
pub static RAW_DATA: Lazy<RawData> = Lazy::new(|| {
    rmp_serde::from_slice::<RawData>(include_bytes!("../../../data/raw_data")).unwrap()
});
