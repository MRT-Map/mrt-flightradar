use color_eyre::eyre::Result;
use tracing::debug;

use crate::{
    data_types::{airport::Runway, vec::FromLoc, waypoint::Waypoint},
    flight_route::{
        flight_path::get_flight_path, types::path::FlightPath, waypoint_route::get_waypoint_route,
    },
};

mod between_waypoints;
mod flight_path;
pub mod types;
mod waypoint_route;

#[tracing::instrument(skip_all)]
pub fn get_flight_route(
    start_runway: &Runway,
    end_runway: &Runway,
) -> Result<(Vec<&'static Waypoint>, FlightPath)> {
    let start_vec = FromLoc {
        tail: start_runway.vec.tail,
        vec: start_runway.vec.vec.normalize() * (500.0 + start_runway.vec.vec.length()),
    };
    let end_vec_vec = end_runway.vec.vec.normalize() * (500.0 + end_runway.vec.vec.length());
    let end_vec = FromLoc {
        tail: end_runway.vec.head() - end_vec_vec,
        vec: end_vec_vec,
    };
    debug!(?start_vec, ?end_vec);

    let (waypoints, positions) = get_waypoint_route(start_vec, end_vec)?;
    Ok((
        waypoints,
        get_flight_path(start_vec, end_vec, positions, 100.0),
    ))
}
