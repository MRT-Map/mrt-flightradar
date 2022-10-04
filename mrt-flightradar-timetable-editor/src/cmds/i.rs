use std::{iter::Peekable, str::Split};

use anyhow::Result;
use common::types::{airport::AirFacility, timetable::AirlineTimetable};

use crate::{arg, Action};

pub fn i(
    cmd_str: &mut Peekable<Split<char>>,
    file: &mut AirlineTimetable,
    air_facilities: &[AirFacility],
) -> Result<Action> {
    let index = arg!(cmd_str "index" get_index, file, le)?;
    let flight = arg!(cmd_str "flight" get_flight, air_facilities)?;
    file.flights.insert(index, flight);
    Ok(Action::Refresh)
}
