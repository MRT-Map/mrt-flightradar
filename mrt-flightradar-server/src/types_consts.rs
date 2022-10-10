use std::{collections::HashMap, sync::Arc, time::SystemTime};

use common::{
    data_types::{timetable::AirportCode, vec::FromLoc},
    flight_route::types::path::FlightPath,
};
use once_cell::sync::Lazy;
use serde::Serialize;
use smol_str::SmolStr;
use tokio::sync::Mutex;

#[derive(Clone, Debug, Serialize)]
pub struct ActiveFlightInfo<'a> {
    pub airline_name: &'a str,
    pub aircraft: &'a str,
    pub registry_code: SmolStr,
    pub from: &'a AirportCode,
    pub to: &'a AirportCode,
}

#[derive(Clone, Debug, Serialize)]
pub struct ActiveFlight<'a> {
    #[serde(skip)]
    pub route: FlightPath,
    pub depart_time: SystemTime,
    pub arrival_time: SystemTime,
    pub info: ActiveFlightInfo<'a>,
}

#[derive(Clone, Debug, Serialize)]
pub struct FlightStatus<'a> {
    pub flight: Arc<ActiveFlight<'a>>,
    pub vec: FromLoc,
}

pub static FLIGHTS: Lazy<Mutex<Vec<Arc<ActiveFlight>>>> = Lazy::new(|| Mutex::new(Vec::new()));
#[allow(clippy::type_complexity)]
pub static FLIGHT_STATUSES: Lazy<Mutex<HashMap<SystemTime, Vec<FlightStatus>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
