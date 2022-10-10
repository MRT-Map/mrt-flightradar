use std::time::{SystemTime, UNIX_EPOCH};

use common::data_types::vec::FromLoc;
use tokio::time::Duration;
use tracing::{debug, info};

use crate::types_consts::{FlightStatus, FLIGHTS, FLIGHT_STATUSES};

#[tracing::instrument]
pub async fn calculate_statuses() {
    let mut flight_statuses = FLIGHT_STATUSES.lock().await;
    let flights = FLIGHTS.lock().await;
    for i in [0, 5, 10, 15, 20, 25] {
        let key = SystemTime::now() + Duration::from_secs(30 + i);
        info!(
            time = key.duration_since(UNIX_EPOCH).unwrap().as_secs(),
            "Calculating statuses"
        );

        let new_flight_statuses = flights
            .iter()
            .filter(|f| f.depart_time < key && key < f.arrival_time)
            .filter_map(|f| {
                if f.depart_time > key || key > f.arrival_time {
                    return None;
                }
                debug!(
                    time = key.duration_since(UNIX_EPOCH).unwrap().as_secs(),
                    "Calculating for flight from {} to {}", f.info.from, f.info.to
                );
                Some(FlightStatus {
                    flight: f.to_owned(),
                    vec: FromLoc::new(
                        f.route
                            .pos_at_time((key.duration_since(f.depart_time).ok()?).as_secs_f32())?,
                        f.route
                            .pos_at_time(
                                key.duration_since(f.depart_time).ok()?.as_secs_f32() + 5.0,
                            )
                            .or_else(|| f.route.pos_at_time(f.route.time_taken()))?,
                    ),
                })
            })
            .collect();
        flight_statuses.insert(key, new_flight_statuses);
        info!("Flight statuses calculated");
    }
}
