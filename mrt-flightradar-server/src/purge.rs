use std::time::SystemTime;

use crate::types_consts::{FLIGHTS, FLIGHT_STATUSES};

pub async fn purge_outdated_data() {
    FLIGHT_STATUSES
        .lock()
        .await
        .retain(|a, _| SystemTime::now() > *a);
    FLIGHTS
        .lock()
        .await
        .retain(|a| SystemTime::now() > a.arrival_time);
}
