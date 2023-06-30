use std::{
    collections::HashMap,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use common::{
    data_types::{timetable::AirportCode, vec::FromLoc, waypoint::Waypoint},
    flight_route::types::path::FlightPath,
};
use once_cell::sync::Lazy;
use serde::{Serialize, Serializer};
use smol_str::SmolStr;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct ActiveFlightInfo<'a> {
    pub airline_name: &'a str,
    pub aircraft: &'a str,
    pub registry_code: SmolStr,
    pub from: &'a AirportCode,
    pub to: &'a AirportCode,
    pub waypoints: Vec<&'static Waypoint>,
}

fn serialise_as_timestamp<S: Serializer>(a: &SystemTime, ser: S) -> Result<S::Ok, S::Error> {
    ser.serialize_u64(a.duration_since(UNIX_EPOCH).unwrap().as_secs())
}

fn serialise_as_str<S: Serializer>(a: &Uuid, ser: S) -> Result<S::Ok, S::Error> {
    ser.serialize_str(&a.to_string())
}

#[derive(Clone, Debug, Serialize)]
pub struct ActiveFlight<'a> {
    #[serde(serialize_with = "serialise_as_str")]
    pub id: Uuid,
    #[serde(skip)]
    pub route: FlightPath,
    #[serde(serialize_with = "serialise_as_timestamp")]
    pub depart_time: SystemTime,
    #[serde(serialize_with = "serialise_as_timestamp")]
    pub arrival_time: SystemTime,
    pub info: ActiveFlightInfo<'a>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
pub enum FlightAction<'a> {
    Add {
        flight: Arc<ActiveFlight<'a>>,
        vec: FromLoc,
    },
    Remove {
        #[serde(serialize_with = "serialise_as_str")]
        id: Uuid,
    },
    Move {
        #[serde(serialize_with = "serialise_as_str")]
        id: Uuid,
        vec: FromLoc,
    },
}

pub static FLIGHTS: Lazy<Mutex<Vec<Arc<ActiveFlight>>>> = Lazy::new(|| Mutex::new(Vec::new()));
#[allow(clippy::type_complexity)]
pub static FLIGHT_ACTIONS: Lazy<Mutex<HashMap<SystemTime, Vec<FlightAction>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
