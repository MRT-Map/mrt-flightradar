mod flight_generation;
mod purge;
mod status_calculation;
mod types_consts;

use std::{collections::HashMap, time::UNIX_EPOCH};

use color_eyre::eyre::Result;
use common::{
    data_types::{vec::Pos, RAW_DATA},
    flight_route::types::path::Path,
};
use glam::Vec2;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    routes,
    serde::json::Json,
    Request, Response,
};
use tokio::time::Duration;
use tracing::error;
use tracing_subscriber::EnvFilter;
use types_consts::FLIGHT_ACTIONS;
use uuid::Uuid;

use crate::{
    flight_generation::generate_flights,
    purge::purge_outdated_data,
    status_calculation::calculate_statuses,
    types_consts::{ActiveFlight, FlightAction, FLIGHTS},
};

#[rocket::get("/actions")]
async fn actions() -> Json<HashMap<String, Vec<FlightAction<'static>>>> {
    FLIGHT_ACTIONS
        .lock()
        .await
        .iter()
        .map(|(k, v)| {
            (
                k.duration_since(UNIX_EPOCH).unwrap().as_secs().to_string(),
                v.to_owned(),
            )
        })
        .collect::<HashMap<_, _>>()
        .into()
}

#[rocket::get("/flights")]
async fn flights() -> Json<Vec<ActiveFlight<'static>>> {
    FLIGHTS
        .lock()
        .await
        .iter()
        .map(|a| (**a).to_owned())
        .collect::<Vec<_>>()
        .into()
}

#[rocket::get("/airports")]
async fn airports() -> Json<HashMap<&'static str, Pos<Vec2>>> {
    RAW_DATA
        .waypoints
        .iter()
        .filter_map(|w| {
            if w.name.starts_with("AA") {
                Some((&w.name[2..], w.coords))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>()
        .into()
}

#[rocket::get("/route/<id>")]
async fn flight_route(id: String) -> Option<Json<Vec<Pos<Vec2>>>> {
    let id = id.parse::<Uuid>().ok()?;
    let flights = FLIGHTS.lock().await;
    let flight = flights.iter().find(|a| a.id == id)?;
    Some(
        flight
            .route
            .0
            .iter()
            .flat_map(|p| match p {
                Path::Straight(fl) => {
                    vec![fl.tail, fl.head()]
                }
                Path::Curve {
                    centre,
                    from,
                    angle,
                } => (0..=36)
                    .map(|i| {
                        *centre
                            + (*from - *centre).rotate(Vec2::from_angle(i as f32 / 36f32 * *angle))
                    })
                    .collect(),
            })
            .collect::<Vec<_>>()
            .into(),
    )
}

// https://stackoverflow.com/questions/62412361/how-to-set-up-cors-or-options-for-rocket-rs
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[rocket::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .event_format(tracing_subscriber::fmt::format().without_time().compact())
        .with_env_filter(EnvFilter::from_env("RUST_LOG"))
        .init();

    let r = rocket::build()
        .mount("/", routes![actions, flights, flight_route, airports])
        .attach(CORS)
        .ignite()
        .await?;

    let h = tokio::spawn(async {
        loop {
            purge_outdated_data().await;
            match generate_flights().await {
                Ok(flights) => calculate_statuses(flights).await,
                Err(e) => error!("{e}"),
            };
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    });
    let _ = r.launch().await?;
    h.abort();
    Ok(())
}
