use std::{sync::Arc, time::SystemTime};

use anyhow::{anyhow, Result};
use common::{
    data_types::{airport::AirFacility, RAW_DATA},
    flight_route::get_flight_route,
};
use rand::{prelude::SliceRandom, Rng};
use tokio::time::Duration;
use tracing::{debug, info};

use crate::types_consts::{ActiveFlight, ActiveFlightInfo, FLIGHTS};

const AIRLINE_NAMES: [&str; 6] = [
    "Example Air",
    "Lorem Ipsum Air",
    "Broken GPS Airlines",
    "0% Reliability Airlines",
    "Gas",
    "Lumeva Airlink",
];

const AIRCRAFT_NAMES: [&str; 4] = ["Stratus SA-1", "IntraJet ExpiXS", "Fighter Jet", "Dragon"];

#[tracing::instrument]
pub async fn generate_flights() -> Result<()> {
    let mut new_flights = vec![];
    let num_new_flights = rand::thread_rng().gen_range(0..5);
    info!("Generating {num_new_flights} new flights");
    for _ in 0..num_new_flights {
        let airports = RAW_DATA
            .air_facilities
            .iter()
            .filter_map(|af| {
                if let AirFacility::Airport { code, ty, runways } = &af {
                    Some((code, ty, runways))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let ((airport1, _, runways1), (airport2, _, runways2)) = {
            let chosen = airports
                .choose_multiple(&mut rand::thread_rng(), 2)
                .collect::<Vec<_>>();
            (chosen[0], chosen[1])
        };
        debug!(?airport1, ?airport2);
        let (runway1, runway2) = (
            runways1
                .choose(&mut rand::thread_rng())
                .ok_or_else(|| anyhow!("No runways"))?,
            runways2
                .choose(&mut rand::thread_rng())
                .ok_or_else(|| anyhow!("No runways"))?,
        );
        let route = get_flight_route(runway1, runway2)?;
        let depart_time = SystemTime::now() + Duration::from_secs(30);
        let arrival_time = depart_time + Duration::from_secs(route.time_taken() as u64);
        new_flights.push(Arc::new(ActiveFlight {
            route,
            depart_time,
            arrival_time,
            info: ActiveFlightInfo {
                airline_name: AIRLINE_NAMES.choose(&mut rand::thread_rng()).unwrap(),
                aircraft: AIRCRAFT_NAMES.choose(&mut rand::thread_rng()).unwrap(),
                registry_code: "".into(),
                from: airport1,
                to: airport2,
            },
        }));
    }
    FLIGHTS.lock().await.append(&mut new_flights);
    info!("New flights inserted");
    Ok(())
}
