use std::time::{Duration, SystemTime};

use tracing::info;

use crate::types_consts::{FLIGHTS, FLIGHT_STATUSES};

#[tracing::instrument]
pub async fn purge_outdated_data() {
    info!("Purging outdated flight statuses");
    FLIGHT_STATUSES
        .lock()
        .await
        .retain(|a, _| SystemTime::now() - Duration::from_secs(15) < *a);
    info!("Purging outdated flights");
    FLIGHTS
        .lock()
        .await
        .retain(|a| SystemTime::now() - Duration::from_secs(15) < a.arrival_time);
}
