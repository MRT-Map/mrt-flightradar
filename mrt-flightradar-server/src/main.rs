mod flight_generation;

use std::{sync::Arc, time::Duration};

use anyhow::Result;
use common::{data_types::timetable::AirportCode, flight_route::types::path::FlightPath};
use once_cell::sync::Lazy;
use rocket::routes;
use smol_str::SmolStr;
use tokio::{sync::Mutex, time::Instant};

use crate::flight_generation::flight_generation;

#[derive(Clone, Debug)]
pub struct ActiveFlightInfo<'a> {
    pub airline_name: &'a str,
    pub aircraft: &'a str,
    pub registry_code: SmolStr,
    pub from: &'a AirportCode,
    pub to: &'a AirportCode,
}

#[derive(Clone, Debug)]
pub struct ActiveFlight<'a> {
    pub route: FlightPath,
    pub depart_time: Instant,
    pub info: ActiveFlightInfo<'a>,
}

static FLIGHTS: Lazy<Arc<Mutex<Vec<ActiveFlight>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

#[rocket::get("/")]
fn test() -> &'static str {
    "abc"
}

#[rocket::main]
async fn main() -> Result<()> {
    tokio::spawn(async {
        loop {
            flight_generation().await.unwrap();
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    });
    let _ = rocket::build().mount("/", routes![test]).launch().await?;
    Ok(())
}
