mod flight_generation;
mod purge;
mod status_calculation;
mod types_consts;

use std::{collections::HashMap, time::UNIX_EPOCH};

use color_eyre::eyre::Result;
use rocket::{routes, serde::json::Json};
use tokio::time::Duration;
use tracing::error;
use tracing_subscriber::EnvFilter;
use types_consts::{FlightStatus, FLIGHT_STATUSES};

use crate::{
    flight_generation::generate_flights, purge::purge_outdated_data,
    status_calculation::calculate_statuses,
};

#[rocket::get("/")]
#[allow(clippy::unnecessary_to_owned)]
async fn test() -> Json<HashMap<String, Vec<FlightStatus<'static>>>> {
    FLIGHT_STATUSES
        .lock()
        .await
        .to_owned()
        .into_iter()
        .map(|(k, v)| {
            (
                k.duration_since(UNIX_EPOCH).unwrap().as_secs().to_string(),
                v,
            )
        })
        .collect::<HashMap<_, _>>()
        .into()
}

#[rocket::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .event_format(tracing_subscriber::fmt::format().without_time().compact())
        .with_env_filter(EnvFilter::from_env("RUST_LOG"))
        .init();
    let r = rocket::build().mount("/", routes![test]).ignite().await?;

    let h = tokio::spawn(async {
        loop {
            purge_outdated_data().await;
            let _ = generate_flights().await.map_err(|e| error!("{e}"));
            calculate_statuses().await;
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    });
    let _ = r.launch().await?;
    h.await?;
    Ok(())
}
