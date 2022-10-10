mod flight_generation;
mod purge;
mod status_calculation;
mod types_consts;

use std::{collections::HashMap, time::SystemTime};

use anyhow::Result;
use rocket::{routes, serde::json::Json};
use tokio::time::Duration;
use tracing::error;
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, FmtSubscriber};
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
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_env("RUST_LOG"))
        .init();
    let r = rocket::build().mount("/", routes![test]).ignite().await?;

    let h = tokio::spawn(async {
        loop {
            let _ = generate_flights().await.map_err(|e| error!("{e}"));
            purge_outdated_data().await;
            calculate_statuses().await;
            tokio::time::sleep(Duration::from_secs(15)).await;
            calculate_statuses().await;
            tokio::time::sleep(Duration::from_secs(15)).await;
        }
    });
    let _ = r.launch().await?;
    h.await?;
    Ok(())
}
