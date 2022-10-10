use std::time::SystemTime;

use common::data_types::vec::FromLoc;
use tokio::time::Duration;
use tracing::info;

use crate::types_consts::{FlightStatus, FLIGHTS, FLIGHT_STATUSES};

#[tracing::instrument]
pub async fn calculate_statuses() {
    let mut flight_statuses = FLIGHT_STATUSES.lock().await;
    let flights = FLIGHTS.lock().await;
    while if let Some(a) = flight_statuses.keys().max() {
        if let Ok(dur) = a.duration_since(SystemTime::now()) {
            dur > Duration::from_secs(45)
        } else {
            true
        }
    } else {
        true
    } {
        let a = SystemTime::now() - Duration::from_secs(15);
        let key = *flight_statuses.keys().max().unwrap_or(&a) + Duration::from_secs(15);
        info!("Calculating for {key:?}");

        let new_flight_statuses = flights
            .iter()
            .filter(|f| f.depart_time < key && key < f.arrival_time)
            .filter_map(|f| {
                if f.depart_time > key || key > f.arrival_time {
                    return None;
                }
                Some(FlightStatus {
                    flight: f.to_owned(),
                    vec: FromLoc::new(
                        f.route
                            .pos_at_time((key.duration_since(f.depart_time).ok()?).as_secs_f32())?,
                        f.route
                            .pos_at_time(
                                key.duration_since(f.depart_time).ok()?.as_secs_f32() + 15.0,
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
