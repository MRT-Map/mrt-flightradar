mod flight_generation;
mod purge;
mod status_calculation;
mod types_consts;

use std::{collections::HashMap, time::SystemTime};

use anyhow::Result;
use rocket::{routes, serde::json::Json};
use tokio::time::Duration;
use types_consts::{FlightStatus, FLIGHT_STATUSES};

use crate::{
    flight_generation::generate_flights, purge::purge_outdated_data,
    status_calculation::calculate_statuses,
};

#[rocket::get("/")]
async fn test() -> Json<HashMap<SystemTime, Vec<FlightStatus<'static>>>> {
    FLIGHT_STATUSES.lock().await.to_owned().into()
}

#[rocket::main]
async fn main() -> Result<()> {
    tokio::spawn(async {
        loop {
            generate_flights().await.unwrap();
            purge_outdated_data().await;
            calculate_statuses().await;
            tokio::time::sleep(Duration::from_secs(15)).await;
            calculate_statuses().await;
            tokio::time::sleep(Duration::from_secs(15)).await;
        }
    });
    let _ = rocket::build().mount("/", routes![test]).launch().await?;
    Ok(())
}
