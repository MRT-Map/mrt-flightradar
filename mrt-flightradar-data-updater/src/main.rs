mod generate_airways;
mod get_air_facilities;
mod get_waypoints;

use std::io::Read;

use color_eyre::eyre::Result;
use common::data_types::RawData;
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::{
    generate_airways::generate_airways, get_air_facilities::get_air_facilities,
    get_waypoints::get_waypoints,
};

const AIR_FACILITY_LIST_URL: &str = "https://docs.google.com/spreadsheets/d/11E60uIBKs5cOSIRHLz0O0nLCefpj7HgndS1gIXY_1hw/export?format=csv";
const WAYPOINT_LIST_URL: &str = "https://docs.google.com/spreadsheets/d/11E60uIBKs5cOSIRHLz0O0nLCefpj7HgndS1gIXY_1hw/export?format=csv&gid=707730663";

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .event_format(tracing_subscriber::fmt::format().without_time().compact())
        .with_env_filter(EnvFilter::from_env("RUST_LOG"))
        .init();

    let air_facilities = {
        let mut str = "".into();
        reqwest::blocking::get(AIR_FACILITY_LIST_URL)?.read_to_string(&mut str)?;
        info!("Air facilities retrieved");
        str
    };
    let waypoints = {
        let mut str = "".into();
        reqwest::blocking::get(WAYPOINT_LIST_URL)?.read_to_string(&mut str)?;
        info!("Waypoints retrieved");
        str
    };
    let air_facilities = get_air_facilities(&*air_facilities)?;
    let waypoints = get_waypoints(&*waypoints)?;
    let airways = generate_airways(&waypoints);
    let airway_coords = airways
        .iter()
        .filter_map(|aw| {
            Some((
                waypoints
                    .iter()
                    .find(|w| w.name == aw.waypoint1)?
                    .coords
                    .to_owned(),
                waypoints
                    .iter()
                    .find(|w| w.name == aw.waypoint2)?
                    .coords
                    .to_owned(),
            ))
        })
        .collect::<Vec<_>>();
    let raw_data = RawData {
        air_facilities,
        waypoints,
        airways,
    };

    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "data/raw_data".into());
    std::fs::write(path, rmp_serde::to_vec(&raw_data)?)?;
    info!("Saved raw_data");

    let path2 = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "data/airway_coords.json".into());
    std::fs::write(path2, serde_json::to_string(&airway_coords)?)?;
    info!("Saved airway_coords.json");

    Ok(())
}
