use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use common::data_types::vec::FromLoc;
use tokio::time::Duration;
use tracing::{debug, info, warn};

use crate::types_consts::{ActiveFlight, FlightAction, FLIGHTS, FLIGHT_ACTIONS};

pub fn calculate_vec(f: &ActiveFlight, key: SystemTime) -> Option<FromLoc> {
    Some(FromLoc::new(
        f.route
            .pos_at_time((key.duration_since(f.depart_time).ok()?).as_secs_f32())?,
        f.route
            .pos_at_time(key.duration_since(f.depart_time).ok()?.as_secs_f32() + 5.0)
            .or_else(|| f.route.pos_at_time(f.route.time_taken()))?,
    ))
}

#[tracing::instrument(skip_all)]
pub async fn calculate_statuses(new_flights: Vec<Arc<ActiveFlight<'static>>>) {
    let mut flight_statuses = FLIGHT_ACTIONS.lock().await;
    let mut flights = FLIGHTS.lock().await;
    for i in [0, 5, 10, 15, 20, 25] {
        let key = SystemTime::now() + Duration::from_secs(30 + i);
        let mut actions = vec![];
        info!(
            time = key.duration_since(UNIX_EPOCH).unwrap().as_secs(),
            "Calculating statuses",
        );

        if i == 0 {
            for flight in &new_flights {
                let vec = if let Some(vec) = calculate_vec(flight, key) {
                    vec
                } else {
                    warn!(
                        "Could not find initial vec for flight from {} to {}",
                        flight.info.from, flight.info.to
                    );
                    continue;
                };
                debug!(
                    "Adding flight from {} to {}",
                    flight.info.from, flight.info.to
                );
                flights.push(flight.to_owned());
                actions.push(FlightAction::Add {
                    flight: flight.to_owned(),
                    vec,
                })
            }
        }

        for flight in &*flights {
            if let Ok(dur) = key.duration_since(flight.arrival_time) {
                if dur < Duration::from_secs(5) {
                    debug!(
                        "Generating remove for flight from {} to {}",
                        flight.info.from, flight.info.to
                    );
                    actions.push(FlightAction::Remove { id: flight.id });
                    continue;
                }
            }
            let vec = if let Some(vec) = calculate_vec(flight, key) {
                vec
            } else {
                warn!(
                    "Could not find vec for flight from {} to {}",
                    flight.info.from, flight.info.to
                );
                actions.push(FlightAction::Remove { id: flight.id });
                continue;
            };

            debug!(
                "Generating movement vec for flight from {} to {}",
                flight.info.from, flight.info.to
            );
            actions.push(FlightAction::Move { id: flight.id, vec });
        }
        flight_statuses.insert(key, actions);
        info!("Flight statuses calculated");
    }
}
