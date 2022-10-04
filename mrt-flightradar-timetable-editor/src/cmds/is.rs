use std::{iter::Peekable, str::Split};

use anyhow::Result;
use common::types::{airport::AirFacility, timetable::AirlineTimetable};

use crate::{arg, Action};

pub fn is(
    cmd_str: &mut Peekable<Split<char>>,
    file: &mut AirlineTimetable,
    air_facilities: &Vec<AirFacility>,
) -> Result<Action> {
    let flight = arg!(cmd_str "flight" get_flight, air_facilities);
    file.flights.insert(0, flight);
    Ok(Action::Refresh)
}
