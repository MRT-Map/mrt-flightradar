use std::time::{Duration, SystemTime};

use tracing::info;

use crate::types_consts::{FLIGHTS, FLIGHT_ACTIONS};

#[tracing::instrument]
pub async fn purge_outdated_data() {
    let mut flight_statuses = FLIGHT_ACTIONS.lock().await;
    info!(
        len = flight_statuses.len(),
        "Purging outdated flight statuses"
    );
    flight_statuses.retain(|a, _| SystemTime::now() - Duration::from_secs(15) < *a);
    let mut flights = FLIGHTS.lock().await;
    info!(len = flight_statuses.len(), "Purged");

    info!(len = flights.len(), "Purging outdated flights");
    flights.retain(|a| SystemTime::now() - Duration::from_secs(15) < a.arrival_time);
    info!(len = flights.len(), "Purged");
}
