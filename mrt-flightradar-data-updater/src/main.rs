mod get_air_facilities;
mod get_waypoints;

use std::io::Read;

use anyhow::Result;
use common::data_types::RawData;

use crate::{get_air_facilities::get_air_facilities, get_waypoints::get_waypoints};

const AIR_FACILITY_LIST_URL: &str = "https://docs.google.com/spreadsheets/d/11E60uIBKs5cOSIRHLz0O0nLCefpj7HgndS1gIXY_1hw/export?format=csv";
const WAYPOINT_LIST_URL: &str = "https://docs.google.com/spreadsheets/d/11E60uIBKs5cOSIRHLz0O0nLCefpj7HgndS1gIXY_1hw/export?format=csv&gid=707730663";

fn main() -> Result<()> {
    let air_facilities = {
        let mut str = "".into();
        reqwest::blocking::get(AIR_FACILITY_LIST_URL)?.read_to_string(&mut str)?;
        str
    };
    let waypoints = {
        let mut str = "".into();
        reqwest::blocking::get(WAYPOINT_LIST_URL)?.read_to_string(&mut str)?;
        str
    };
    let raw_data = RawData {
        air_facilities: get_air_facilities(&*air_facilities)?,
        waypoints: get_waypoints(&*waypoints)?,
    };
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "data/raw_data".into());
    std::fs::write(path, rmp_serde::to_vec(&raw_data)?)?;

    Ok(())
}
